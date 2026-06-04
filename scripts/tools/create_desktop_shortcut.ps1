$ErrorActionPreference = "Stop"

$WshShell = New-Object -ComObject WScript.Shell
$Desktop = [Environment]::GetFolderPath("Desktop")
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

function Find-Launcher {
    param([string]$Root)
    $preferred = @(
        (Join-Path $Root "► ЗАПУСК JARVIS.vbs"),
        (Join-Path $Root "start-jarvis.vbs")
    )
    foreach ($path in $preferred) {
        if (Test-Path -LiteralPath $path) {
            return $path
        }
    }
    $any = Get-ChildItem -LiteralPath $Root -Filter "*.vbs" -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -match "jarvis|JARVIS|запуск" } |
        Select-Object -First 1
    if ($any) {
        return $any.FullName
    }
    return $null
}

$Launcher = Find-Launcher -Root $ProjectRoot
if (-not $Launcher) {
    Write-Error "Launcher VBS not found in $ProjectRoot"
}

$iconIco = Join-Path $ProjectRoot "resources\icons\icon.ico"
$iconExe = Join-Path $ProjectRoot "target\release\jarvis-gui.exe"
if (-not (Test-Path -LiteralPath $iconIco)) {
    $iconIco = $iconExe
}

$shortcutPath = Join-Path $Desktop "JARVIS.lnk"
$shortcut = $WshShell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = "$env:SystemRoot\System32\wscript.exe"
$shortcut.Arguments = "`"$Launcher`""
$shortcut.WorkingDirectory = $ProjectRoot
$shortcut.Description = "JARVIS voice assistant"
if (Test-Path -LiteralPath $iconIco) {
    $shortcut.IconLocation = "$iconIco,0"
}
$shortcut.Save()

Write-Host "Desktop shortcut: $shortcutPath"
Write-Host "Launcher: $Launcher"
