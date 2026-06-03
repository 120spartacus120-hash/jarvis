$root = "C:\Users\Asus\Desktop\EXE\Jarvis\jarvis-master"
$exe = Join-Path $root "target\release\jarvis-gui.exe"
$idx = Join-Path $root "frontend\dist\client\index.html"
$src = Join-Path $root "frontend\src\lib\customCommands.ts"

Write-Host "SRC open_website:" (Select-String -Path $src -Pattern "open_website" -Quiet)
Write-Host "EXE exists:" (Test-Path $exe)
if (Test-Path $exe) { Write-Host "EXE time:" (Get-Item $exe).LastWriteTime }
if (Test-Path $idx) { Write-Host "INDEX time:" (Get-Item $idx).LastWriteTime }

$hits = Get-ChildItem (Join-Path $root "frontend\dist\client\assets\*.js") | ForEach-Object {
    if (Select-String -Path $_.FullName -Pattern "open_website" -Quiet) { $_.Name }
}
Write-Host "JS with open_website:" ($hits -join ", ")
if (-not $hits) { Write-Host "DIST MISSING open_website - need npm run build" }
