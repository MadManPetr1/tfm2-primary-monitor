# Changelog

All notable public changes to Primary Monitor will be documented here.

The project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.0] - 2026-08-05

### Changed

- Standardized the Cargo crate, mod ID, installed folder, and DLL as
  `tfm2_primary_monitor`.
- Kept the existing one-time window placement behavior unchanged.

## [0.1.5] - 2026-08-05

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.4` Mod SDK.
- Extended the supported base range to `>=0.5.2, <0.5.5` without changing
  one-time window placement behavior.

### Compatibility

- Runtime-tested on Teamfight Manager 2 `0.5.4`.

## [0.1.4] - 2026-08-03

### Added

- Unified Better Mod Menu author profile and profile icon.
- New 256 px pixel-art monitor thumbnail.

### Changed

- Simplified player documentation and release presentation.
- Cleaned source and release packaging without changing placement behavior.

### Compatibility

- Window-placement behavior and the tested TFM2 `0.5.2`-`0.5.3` range are
  unchanged.

## [0.1.3] - 2026-07-30

### Changed

- Added an explicit `primary_monitor` package identity so manual and Workshop
  installations resolve the same mod ID without relying on the folder name.

### Compatibility

- Revalidated the compatibility-baseline build on Teamfight Manager 2 `0.5.3`.

## [0.1.2] - 2026-07-29

### Changed

- Expanded and standardized the public documentation, contribution guidance,
  support information, and release presentation.
- Relicensed new project versions under MPL-2.0 so distributed changes to
  covered files remain shareable.
- Reserved the original project branding from the source-code license.
- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.2` Mod SDK as
  the compatibility baseline.
- Expanded the supported game range to `>=0.5.2, <0.5.4`.
- Replaced repeated window-title allocations with a static UTF-16 title.
- Throttled failed startup window lookups instead of calling Win32 every frame.

### Compatibility

- Verified the compatibility-baseline build against Teamfight Manager 2
  `0.5.3`.

## [0.1.1] - 2026-07-28

### Changed

- Rebuilt the native mod against the Teamfight Manager 2 `0.5.2` Mod SDK.
- Updated the supported game range to `>=0.5.2, <0.5.3`.

## [0.1.0] - 2026-07-27

### Added

- One-time launch positioning on the Windows primary monitor.
- Centering within the primary monitor's usable work area while preserving window size.
- No interference with later manual window movement.

[Unreleased]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/MadManPetr1/tfm2-primary-monitor/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/MadManPetr1/tfm2-primary-monitor/tree/v0.1.0
