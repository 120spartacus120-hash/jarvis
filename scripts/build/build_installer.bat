@echo off
chcp 65001 >nul
setlocal EnableExtensions

set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

echo.
echo ========================================
echo   JARVIS - NSIS installer build
echo ========================================
echo   Project: %ROOT%
echo.

taskkill /IM jarvis-gui.exe /F >nul 2>&1
taskkill /IM jarvis-app.exe /F >nul 2>&1

echo [1/4] Clean frontend dist...
if exist "%ROOT%frontend\dist" rd /s /q "%ROOT%frontend\dist"

echo [2/4] Frontend release build...
cd /d "%ROOT%frontend"
call npm run build
if errorlevel 1 exit /b 1

echo [3/4] jarvis-app release + native libs...
cd /d "%ROOT%"
cargo build --release -p jarvis-app
if errorlevel 1 exit /b 1
python "%ROOT%scripts\tools\post_build.py" --force
if errorlevel 1 exit /b 1

echo [4/4] Tauri NSIS installer (Russian wizard)...
cd /d "%ROOT%"
call npm exec --prefix frontend tauri -- build --config crates/jarvis-gui/tauri.conf.json --ignore-version-mismatches
if errorlevel 1 exit /b 1

set "NSIS_DIR=%ROOT%target\release\bundle\nsis"
set "SETUP="
for %%F in ("%NSIS_DIR%\*-setup.exe") do set "SETUP=%%~fF"

if not defined SETUP (
    echo [ERROR] NSIS setup not found in %NSIS_DIR%
    exit /b 1
)

set "DESK=%USERPROFILE%\Desktop"
for %%F in ("%SETUP%") do set "BASENAME=%%~nxF"
copy /Y "%SETUP%" "%DESK%\%BASENAME%"

if defined WINDOWS_CODESIGN_PFX_PATH (
    echo [sign] Code signing...
    powershell -NoProfile -ExecutionPolicy Bypass -File "%ROOT%scripts\build\sign_windows.ps1" "%SETUP%"
    if errorlevel 1 exit /b 1
    copy /Y "%SETUP%" "%DESK%\%BASENAME%"
) else (
    echo [sign] Skip: set WINDOWS_CODESIGN_PFX_PATH for Authenticode ^(removes SmartScreen warnings^)
)

echo.
echo Done. Installer on Desktop:
echo   %BASENAME%
echo.
echo Publish to users: upload this *-setup.exe to GitHub Releases.
echo Do NOT distribute old MSI 0.0.3 or zip from target\release\.
echo.
exit /b 0
