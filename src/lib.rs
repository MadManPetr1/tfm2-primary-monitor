use mod_api_stable::{declare_stable_mod, StableClient, StableExtension, StableHost, StableMod};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const MOD_ID: &str = "tfm2_primary_monitor";
const RETRY_INTERVAL_FRAMES: usize = 30;
const GAME_WINDOW_TITLE: &[u16] = &[
    84, 101, 97, 109, 102, 105, 103, 104, 116, 32, 77, 97, 110, 97, 103, 101, 114, 50, 0,
];

struct PrimaryMonitorExtension {
    positioned: AtomicBool,
    retry_frames: AtomicUsize,
}

impl StableExtension for PrimaryMonitorExtension {
    fn post_update(&self, _ctx: &mut StableClient<'_>, _dt_micros: u64) {
        self.attempt_placement(move_game_window_to_primary);
    }
}

impl PrimaryMonitorExtension {
    fn attempt_placement(&self, place: impl FnOnce() -> bool) {
        if self.positioned.load(Ordering::Relaxed) {
            return;
        }

        let retry_frame = self.retry_frames.fetch_add(1, Ordering::Relaxed);
        if !retry_frame.is_multiple_of(RETRY_INTERVAL_FRAMES) {
            return;
        }

        if place() {
            self.positioned.store(true, Ordering::Relaxed);
        }
    }
}

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct MonitorInfo {
    size: u32,
    monitor: Rect,
    work: Rect,
    flags: u32,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn FindWindowW(class_name: *const u16, window_name: *const u16) -> *mut c_void;
    fn GetMonitorInfoW(monitor: *mut c_void, info: *mut MonitorInfo) -> i32;
    fn GetWindowRect(window: *mut c_void, rect: *mut Rect) -> i32;
    fn MonitorFromPoint(point: Point, flags: u32) -> *mut c_void;
    fn SetWindowPos(
        window: *mut c_void,
        insert_after: *mut c_void,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: u32,
    ) -> i32;
}

fn game_window() -> *mut c_void {
    unsafe { FindWindowW(std::ptr::null(), GAME_WINDOW_TITLE.as_ptr()) }
}

fn move_game_window_to_primary() -> bool {
    let window = game_window();
    if window.is_null() {
        return false;
    }

    const MONITOR_DEFAULTTOPRIMARY: u32 = 1;
    let monitor = unsafe { MonitorFromPoint(Point { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY) };
    if monitor.is_null() {
        return false;
    }

    let mut monitor_info = MonitorInfo {
        size: std::mem::size_of::<MonitorInfo>() as u32,
        monitor: Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        work: Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        flags: 0,
    };
    let mut window_rect = Rect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    if unsafe { GetMonitorInfoW(monitor, &mut monitor_info) } == 0
        || unsafe { GetWindowRect(window, &mut window_rect) } == 0
    {
        return false;
    }

    let width = window_rect.right - window_rect.left;
    let height = window_rect.bottom - window_rect.top;
    let work_width = monitor_info.work.right - monitor_info.work.left;
    let work_height = monitor_info.work.bottom - monitor_info.work.top;
    if width <= 0 || height <= 0 || work_width <= 0 || work_height <= 0 {
        return false;
    }

    let x = monitor_info.work.left + ((work_width - width) / 2).max(0);
    let y = monitor_info.work.top + ((work_height - height) / 2).max(0);
    const SWP_NOSIZE: u32 = 0x0001;
    const SWP_NOZORDER: u32 = 0x0004;
    const SWP_NOACTIVATE: u32 = 0x0010;
    unsafe {
        SetWindowPos(
            window,
            std::ptr::null_mut(),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        ) != 0
    }
}

fn init(_host: &StableHost) -> StableMod {
    let mut registration = StableMod::new(MOD_ID);
    registration.set_extension(PrimaryMonitorExtension {
        positioned: AtomicBool::new(false),
        retry_frames: AtomicUsize::new(0),
    });
    registration
}

declare_stable_mod!(init);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retries_at_interval_and_stops_after_success() {
        let extension = PrimaryMonitorExtension {
            positioned: AtomicBool::new(false),
            retry_frames: AtomicUsize::new(0),
        };
        let mut calls = 0;
        for _ in 0..100 {
            extension.attempt_placement(|| {
                calls += 1;
                calls == 2
            });
        }
        assert_eq!(calls, 2);
        assert!(extension.positioned.load(Ordering::Relaxed));
        assert_eq!(extension.retry_frames.load(Ordering::Relaxed), 31);
    }

    #[test]
    fn failed_placement_remains_retryable() {
        let extension = PrimaryMonitorExtension {
            positioned: AtomicBool::new(false),
            retry_frames: AtomicUsize::new(0),
        };
        let mut calls = 0;
        for _ in 0..61 {
            extension.attempt_placement(|| {
                calls += 1;
                false
            });
        }
        assert_eq!(calls, 3);
        assert!(!extension.positioned.load(Ordering::Relaxed));
    }
}
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
