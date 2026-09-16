<div align="center">

<img src="thumbnail.png" alt="Primary Monitor icon" width="128">

# Primary Monitor

Predictable startup placement for **Teamfight Manager 2** on Windows.

**Primary Monitor 0.3.0 · TFM2 0.6.0 · Windows**

</div>

Primary Monitor moves the game window to the Windows primary monitor and
centers it once during startup.

## Behavior

- Preserves the current window size.
- Uses the primary monitor's usable work area.
- Retries failed startup lookups at a bounded interval.
- Stops after the first successful placement, leaving later manual movement alone.
- Never reads saves, uses the network, or continuously watches the window.

It does not change resolution, display mode, fullscreen behavior, or choose a
monitor by number.

## Install

### Steam Workshop

Subscribe through the Teamfight Manager 2 Workshop, enable **Primary Monitor**,
and restart when prompted.

### GitHub release

1. Download `tfm2-primary-monitor-v0.3.0.zip` from
   [GitHub Releases](https://github.com/MadManPetr1/tfm2-primary-monitor/releases).
   Do not use GitHub's automatic source-code archive.
2. Extract `tfm2_primary_monitor` into:

   ```text
   ...\SteamLibrary\steamapps\common\Teamfight Manager2\mods\
   ```

3. Enable **Primary Monitor** and restart the game.

## Compatibility

- Teamfight Manager 2 `0.6.0`
- Windows
- Release DLL uses the Stable API included with TFM2 `0.6.0`
- Game window title `Teamfight Manager2`

## Troubleshooting

For a reproducible problem, include the game/mod versions, Windows display
layout, primary monitor, scaling, window mode, resolution, and the smallest
enabled-mod list that still reproduces it.

## Build

```powershell
.\build_local.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk-stable"
.\scripts\validate_repo.ps1
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk-stable"
```

## License

Source code, scripts, and documentation are licensed under the
[Mozilla Public License 2.0](LICENSE). Original project branding and thumbnail
artwork are not covered by MPL-2.0; see [NOTICE.md](NOTICE.md).

This independent community mod is not affiliated with or endorsed by Team
Samoyed.
