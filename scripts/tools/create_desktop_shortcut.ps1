$WshShell = New-Object -ComObject WScript.Shell
$Desktop = [Environment]::GetFolderPath('Desktop')
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$VbsLauncher = Join-Path $ProjectRoot "► ЗАПУСК JARVIS.vbs"

if (-not (Test-Path $VbsLauncher)) {
    Write-Host "Не найден: $VbsLauncher"
    exit 1
}

$Shortcut = $WshShell.CreateShortcut("$Desktop\JARVIS (запуск).lnk")
$Shortcut.TargetPath = "wscript.exe"
$Shortcut.Arguments = "`"$VbsLauncher`""
$Shortcut.WorkingDirectory = $ProjectRoot
$Shortcut.Description = "JARVIS — актуальная сборка из папки проекта"
$Shortcut.IconLocation = Join-Path $ProjectRoot "resources\icons\icon.ico"
$Shortcut.Save()

Write-Host "Ярлык создан: $Desktop\JARVIS (запуск).lnk"
Write-Host "Он запускает: $VbsLauncher"
