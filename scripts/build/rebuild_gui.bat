@echo off
setlocal EnableExtensions
chcp 65001 >nul

set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

echo [1/3] Stop JARVIS...
taskkill /IM jarvis-gui.exe /F >nul 2>&1
taskkill /IM jarvis-app.exe /F >nul 2>&1
timeout /t 2 /nobreak >nul

echo [2/3] Frontend clean build...
cd /d "%ROOT%frontend"
if exist "dist" rd /s /q "dist"
call npm run build
if errorlevel 1 exit /b 1

echo [3/3] Native libs + jarvis-gui release...
cd /d "%ROOT%"
python "%ROOT%scripts\tools\post_build.py" --force
cargo build -p jarvis-gui --release
if errorlevel 1 exit /b 1

echo.
echo Ready: %ROOT%target\release\jarvis-gui.exe
for %%F in ("%ROOT%target\release\jarvis-gui.exe") do echo Time: %%~tF
echo Close tray and run your VBS launcher.
exit /b 0
