# Contributing

Bug reports, compatibility findings, documentation improvements, and focused
pull requests are welcome.

## Contribution terms

By submitting a contribution, you agree to license it under the
[Mozilla Public License 2.0](LICENSE). Submit only work you created or have the
right to contribute.

MPL-2.0 does not grant trademark rights in the Primary Monitor name. The
original thumbnail artwork is All Rights Reserved. Forks should use their own
name and artwork.

## Before opening an issue

1. Confirm the game and mod versions.
2. Record the Windows display layout, primary monitor, window mode, and
   resolution.
3. Reproduce with the smallest practical enabled-mod list.
4. Remove private information from screenshots and logs.

## Pull requests

- Keep changes focused and preserve the one-time placement behavior.
- Run `cargo fmt --check`.
- Run `.\scripts\validate_repo.ps1` on Windows.
- Test against the declared Teamfight Manager 2 version when runtime behavior
  changes.
- Update `CHANGELOG.md` under **Unreleased**.
- Do not add telemetry, networking, save access, or persistent window control
  without a clear, reviewed requirement.

The native mod depends on the matching Teamfight Manager 2 Mod SDK, which is
not redistributed in this repository.
