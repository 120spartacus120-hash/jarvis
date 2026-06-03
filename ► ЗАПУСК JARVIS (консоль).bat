@echo off
chcp 65001 >nul
setlocal

set "ROOT=%~dp0"
set "RELEASE=%ROOT%target\release"
set "EXE=%RELEASE%\jarvis-gui.exe"

if not exist "%EXE%" (
    set "RELEASE=%ROOT%target\debug"
    set "EXE=%RELEASE%\jarvis-gui.exe"
)

if not exist "%EXE%" (
    echo JARVIS не собран. Запустите: ► СОБРАТЬ JARVIS.bat
    pause
    exit /b 1
)

for %%F in ("%EXE%") do echo Запуск: %%~fF  ^(%%~tF^)

subst J: /d >nul 2>&1
subst J: "%RELEASE%"
if errorlevel 1 (
    echo Не удалось subst J:. Запустите от администратора.
    pause
    exit /b 1
)

cd /d J:\
start "" "J:\jarvis-gui.exe"
exit /b 0
