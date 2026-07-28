# Primary Monitor

Small, predictable window placement for **Teamfight Manager 2** on Windows.

[Features](#features) · [Installation](#installation) · [Behavior](#behavior) · [Building](#building-from-source)

> [!IMPORTANT]
> Version **0.1.1** is built for Teamfight Manager 2 **0.5.2**.

## Features

- Moves the Teamfight Manager 2 window to the Windows primary monitor once
  during startup.
- Centers the window inside the monitor's usable work area.
- Preserves the current window size.
- Stops intervening after the first successful placement, so later manual
  movement remains untouched.

## Behavior

Primary Monitor waits until the game window exists, reads its current size,
finds the Windows primary monitor, and centers the window once. It does not:

- change resolution or display mode;
- force the game to remain on one monitor;
- watch or repeatedly reposition the window;
- read, write, or inspect career saves;
- send data over the network.

## Installation

### Steam Workshop

Subscribe on the Teamfight Manager 2 Workshop, enable **Primary Monitor** in
the in-game Mods menu, then restart the game when prompted.

### Manual GitHub release

1. Download `primary-monitor-vX.Y.Z.zip` from this repository's
   [Releases](https://github.com/MadManPetr1/tfm2-primary-monitor/releases).
   Do not download GitHub's automatic “Source code” archive.
2. Extract the included `primary_monitor` folder into:

   ```text
   ...\SteamLibrary\steamapps\common\Teamfight Manager2\mods\
   ```

3. Confirm this structure:

   ```text
   Teamfight Manager2\mods\primary_monitor\mod.mod_info
   Teamfight Manager2\mods\primary_monitor\primary_monitor.dll
   ```

4. Enable **Primary Monitor** in the Mods menu and restart the game.

## Requirements and limitations

- Teamfight Manager 2 `0.5.2`
- Windows
- The matching `0.5.2` Mod SDK for source builds
- The game window title must be `Teamfight Manager2`

Primary Monitor uses the Windows primary-monitor setting. It does not choose a
monitor by number, remember a custom monitor, or alter fullscreen behavior.

## Building from source

The Mod SDK is not redistributed here. Install the matching SDK with the game,
then run:

```powershell
.\build_local.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

To validate and create a player-ready archive:

```powershell
.\scripts\validate_repo.ps1
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

## Project layout

- `src/lib.rs` — one-time Windows window-placement logic
- `mod.mod_info` — mod metadata and supported game range
- `build_local.ps1` — SDK-aware native build
- `scripts/` — repository validation and release packaging
- `thumbnail.png` — current in-game and Workshop thumbnail

## Support

For a reproducible problem, open a bug report with:

- game and Primary Monitor versions;
- Windows version and display layout;
- window mode and resolution;
- the smallest enabled-mod list that still reproduces the problem;
- expected and actual window position.

## License and attribution

New versions of the original source code, scripts, and documentation are
released under the [Mozilla Public License 2.0](LICENSE). Distributed changes
to covered files must remain available under MPL-2.0. Earlier tagged releases
remain under the license shipped with those releases.

The Primary Monitor name and original branding are reserved and are not
licensed under MPL-2.0; see [NOTICE](NOTICE.md).

This is an independent community mod and is not affiliated with or endorsed by
Team Samoyed.
