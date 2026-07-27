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
    "LICENSE"
)) {
    $path = Join-Path $root $relativePath
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Required file is missing: $relativePath"
    }
}

$modInfo = Get-Content -LiteralPath (Join-Path $root "mod.mod_info") -Raw | ConvertFrom-Json
$cargo = Get-Content -LiteralPath (Join-Path $root "Cargo.toml") -Raw
$cargoVersion = [regex]::Match($cargo, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if ($cargoVersion -ne $modInfo.version) {
    throw "Version mismatch: Cargo.toml is $cargoVersion, mod.mod_info is $($modInfo.version)."
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
