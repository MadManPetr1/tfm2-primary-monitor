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

$dll = Join-Path $root "primary_monitor.dll"
if (-not (Test-Path -LiteralPath $dll -PathType Leaf)) {
    throw "primary_monitor.dll is missing."
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
$buildRoot = Join-Path $root "builds"
$releaseRoot = Join-Path $buildRoot "primary-monitor-v$($modInfo.version)"
$runtimeRoot = Join-Path $releaseRoot "primary_monitor"
$archive = Join-Path $buildRoot "primary-monitor-v$($modInfo.version).zip"

if (Test-Path -LiteralPath $releaseRoot) {
    Remove-Item -LiteralPath $releaseRoot -Recurse -Force
}
if (Test-Path -LiteralPath $archive) {
    Remove-Item -LiteralPath $archive -Force
}
New-Item -ItemType Directory -Path $runtimeRoot -Force | Out-Null

foreach ($name in @(
    "primary_monitor.dll",
    "mod.mod_info",
    "mod.override_info",
    "thumbnail.png",
    "README.md",
    "CHANGELOG.md",
    "LICENSE"
)) {
    Copy-Item -LiteralPath (Join-Path $root $name) -Destination (Join-Path $runtimeRoot $name)
}

Compress-Archive -LiteralPath $runtimeRoot -DestinationPath $archive -CompressionLevel Optimal
Write-Host "Release package created: $archive"
