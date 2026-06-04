# Подпись exe/msi Authenticode (убирает «неизвестный издатель» после накопления репутации).
# Переменные:
#   WINDOWS_CODESIGN_PFX_PATH — путь к .pfx
#   WINDOWS_CODESIGN_PASSWORD — пароль
#   WINDOWS_CODESIGN_TIMESTAMP_URL — по умолчанию http://timestamp.digicert.com

param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$FilePath
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $FilePath)) {
    Write-Error "File not found: $FilePath"
}

$pfx = $env:WINDOWS_CODESIGN_PFX_PATH
$pass = $env:WINDOWS_CODESIGN_PASSWORD
$ts = if ($env:WINDOWS_CODESIGN_TIMESTAMP_URL) { $env:WINDOWS_CODESIGN_TIMESTAMP_URL } else { "http://timestamp.digicert.com" }

if (-not $pfx -or -not (Test-Path $pfx)) {
    Write-Host "[sign] Skip: set WINDOWS_CODESIGN_PFX_PATH to your .pfx certificate"
    exit 0
}

$signtool = Get-Command signtool.exe -ErrorAction SilentlyContinue
if (-not $signtool) {
    $kits = "${env:ProgramFiles(x86)}\Windows Kits\10\bin"
    if (Test-Path $kits) {
        $signtool = Get-ChildItem -Path $kits -Recurse -Filter signtool.exe |
            Where-Object { $_.FullName -match "x64\\signtool.exe" } |
            Sort-Object FullName -Descending |
            Select-Object -First 1
    }
}

if (-not $signtool) {
    Write-Error "signtool.exe not found. Install Windows SDK (Signing Tools)."
}

Write-Host "[sign] Signing: $FilePath"
& $signtool.FullName sign `
    /f $pfx `
    /p $pass `
    /fd sha256 `
    /tr $ts `
    /td sha256 `
    /d "JARVIS Voice Assistant" `
    $FilePath

if ($LASTEXITCODE -ne 0) {
    Write-Error "signtool failed with exit code $LASTEXITCODE"
}

Write-Host "[sign] OK"
