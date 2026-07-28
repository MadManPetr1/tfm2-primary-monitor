# Release guide

## 1. Verify

- Test on the declared Teamfight Manager 2 version.
- Test windowed, borderless, and fullscreen startup.
- Test with multiple displays when runtime behavior changed.
- Run `.\scripts\validate_repo.ps1`.

Do not broaden the version range in `mod.mod_info` until that game version has
been tested.

## 2. Package

```powershell
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

Inspect `builds\primary-monitor-vX.Y.Z.zip`. Its top-level folder must be
`primary_monitor`, and that folder must contain `primary_monitor.dll`.

## 3. Publish

- Use `docs/PRESENTATION.md` for the repository description, release copy, and
  Workshop copy.
- Tag the exact source used to build the archive.
- Attach the matching archive to the GitHub release.
- Install the archive once into a clean mod folder before publishing.
- Keep the Workshop and GitHub version numbers aligned.
