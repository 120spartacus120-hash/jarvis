# Full rebuild after code changes: frontend -> embed in jarvis-gui -> jarvis-app
$ErrorActionPreference = "Stop"
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path

Write-Host "[1/4] Stop JARVIS..." -ForegroundColor Cyan
Get-Process jarvis-gui, jarvis-app -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 2

Write-Host "[2/4] Frontend (clean dist)..." -ForegroundColor Cyan
Set-Location (Join-Path $Root "frontend")
if (Test-Path dist) { Remove-Item -Recurse -Force dist }
npm run build
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "[3/4] Copy native libs (vosk)..." -ForegroundColor Cyan
Set-Location $Root
python (Join-Path $Root "scripts\tools\post_build.py") --force

Write-Host "[4/4] Rust release (jarvis-gui + jarvis-app)..." -ForegroundColor Cyan
cargo build -p jarvis-gui -p jarvis-app --release
if ($LASTEXITCODE -ne 0) { exit 1 }

$gui = Join-Path $Root "target\release\jarvis-gui.exe"
$app = Join-Path $Root "target\release\jarvis-app.exe"
Write-Host ""
Write-Host "OK:" -ForegroundColor Green
Write-Host "  $gui"
Write-Host "  $app"
Write-Host ""
Write-Host "Launch: VBS in project root or jarvis-gui.exe from target\release" -ForegroundColor Yellow
