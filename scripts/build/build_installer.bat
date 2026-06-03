@echo off
chcp 65001 >nul
setlocal EnableExtensions

set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

echo.
echo ========================================
echo   JARVIS - clean installer build
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

echo [3/4] jarvis-app release + resources...
cd /d "%ROOT%"
cargo build --release -p jarvis-app
if errorlevel 1 exit /b 1
python "%ROOT%scripts\tools\post_build.py" --force
if errorlevel 1 exit /b 1

echo [4/4] Tauri bundles (MSI + NSIS)...
cd /d "%ROOT%crates\jarvis-gui"
call npx --yes @tauri-apps/cli@2 build
if errorlevel 1 exit /b 1

cd /d "%ROOT%"
set "MSI=%ROOT%target\release\bundle\msi\JARVIS_1.0.0_x64_en-US.msi"
set "EXE=%ROOT%target\release\bundle\nsis\JARVIS_1.0.0_x64-setup.exe"
set "DESK=%USERPROFILE%\Desktop"

if not exist "%MSI%" (
    echo [ERROR] MSI not found: %MSI%
    exit /b 1
)

copy /Y "%MSI%" "%DESK%\JARVIS_1.0.0_Installer.msi"
if exist "%EXE%" copy /Y "%EXE%" "%DESK%\JARVIS_1.0.0_Setup.exe"

echo.
echo Done. On Desktop:
echo   JARVIS_1.0.0_Installer.msi
echo   JARVIS_1.0.0_Setup.exe
echo.
echo Do NOT install old *0.0.3* MSI from bundle\msi\
echo.
exit /b 0
