use mod_api::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

const MOD_ID: &str = "primary_monitor";

struct PrimaryMonitorExtension {
    positioned: AtomicBool,
}

impl ModExtension for PrimaryMonitorExtension {
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !self.positioned.load(Ordering::Acquire) && move_game_window_to_primary() {
            self.positioned.store(true, Ordering::Release);
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
    let window_title: Vec<u16> = "Teamfight Manager2"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    unsafe { FindWindowW(std::ptr::null(), window_title.as_ptr()) }
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

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut registration = ModRegistration::new(MOD_ID);
    registration.set_extension(PrimaryMonitorExtension {
        positioned: AtomicBool::new(false),
    });
    registration
}

declare_mod!(init);
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
