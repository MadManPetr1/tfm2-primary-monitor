# Primary Monitor

A focused Teamfight Manager 2 code mod that centers the game window on the
Windows primary monitor once during launch.

The mod preserves the current window size and does not interfere if the player
moves the window afterward.

## Compatibility

- Teamfight Manager 2 `0.5.2`
- Windows
- Mod SDK `0.5.2` for source builds

## Installation

Copy the `primary_monitor` folder into the game's `mods` folder, enable
**Primary Monitor** in the Mods menu, and restart the game.

## Building

```powershell
.\build_local.ps1 -SdkDir "D:\Games\Teamfight Manager2\mod-sdk"
```
