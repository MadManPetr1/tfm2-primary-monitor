# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

param(
    [string]$SdkDir = $env:TFM2_MOD_SDK,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

Add-Type -AssemblyName System.Drawing
& (Join-Path $PSScriptRoot "validate_repo.ps1")

if (-not $SkipBuild) {
    & (Join-Path $root "build_local.ps1") -SdkDir $SdkDir
    if ($LASTEXITCODE -ne 0) {
        throw "Release build failed with exit code $LASTEXITCODE."
    }
}

$dll = Join-Path $root "tfm2_primary_monitor.dll"
if (-not (Test-Path -LiteralPath $dll -PathType Leaf)) {
    throw "tfm2_primary_monitor.dll is missing."
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
$buildRoot = Join-Path $root "builds"
$releaseRoot = Join-Path $buildRoot "tfm2-primary-monitor-v$($modInfo.version)"
$runtimeRoot = Join-Path $releaseRoot "tfm2_primary_monitor"
$archive = Join-Path $buildRoot "tfm2-primary-monitor-v$($modInfo.version).zip"

if (Test-Path -LiteralPath $releaseRoot) {
    Remove-Item -LiteralPath $releaseRoot -Recurse -Force
}
if (Test-Path -LiteralPath $archive) {
    Remove-Item -LiteralPath $archive -Force
}
New-Item -ItemType Directory -Path $runtimeRoot -Force | Out-Null

foreach ($name in @(
    "tfm2_primary_monitor.dll",
    "mod.mod_info",
    "mod.override_info",
    "better_mod_menu_profile.json",
    "profile_icon.png",
    "thumbnail.png",
    "README.md",
    "CHANGELOG.md",
    "LICENSE",
    "NOTICE.md"
)) {
    Copy-Item -LiteralPath (Join-Path $root $name) -Destination (Join-Path $runtimeRoot $name)
}

Compress-Archive -LiteralPath $runtimeRoot -DestinationPath $archive -CompressionLevel Optimal
Write-Host "Release package created: $archive"
