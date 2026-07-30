# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param()

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Add-Type -AssemblyName System.Drawing

foreach ($relativePath in @(
    "Cargo.toml",
    "Cargo.lock",
    "src/lib.rs",
    "mod.mod_info",
    "mod.override_info",
    "thumbnail.png",
    "README.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "LICENSE",
    "NOTICE.md",
    "docs/PRESENTATION.md",
    "docs/RELEASING.md",
    "scripts/package_release.ps1"
)) {
    $path = Join-Path $root $relativePath
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Required file is missing: $relativePath"
    }
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
if ($modInfo.mod_id -ne "primary_monitor") {
    throw "mod.mod_info must declare mod_id primary_monitor."
}
$cargo = Get-Content -LiteralPath (Join-Path $root "Cargo.toml") -Raw
$cargoVersion = [regex]::Match($cargo, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if ($cargoVersion -ne $modInfo.version) {
    throw "Version mismatch: Cargo.toml is $cargoVersion, mod.mod_info is $($modInfo.version)."
}
if ($cargo -notmatch '(?m)^license\s*=\s*"MPL-2\.0"') {
    throw "Cargo.toml must declare MPL-2.0."
}

$image = [System.Drawing.Image]::FromFile((Join-Path $root "thumbnail.png"))
try {
    if ($image.Width -ne 512 -or $image.Height -ne 512) {
        throw "thumbnail.png must be 512x512."
    }
}
finally {
    $image.Dispose()
}

Push-Location $root
try {
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) {
        throw "cargo fmt --check failed."
    }
}
finally {
    Pop-Location
}

Write-Host "Repository validation passed for Primary Monitor v$($modInfo.version)."
