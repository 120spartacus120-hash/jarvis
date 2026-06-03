@echo off
setlocal
set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if errorlevel 1 (
    echo [ERROR] vcvars64.bat failed
    exit /b 1
)

echo [1/4] Building frontend...
cd /d "%ROOT%frontend"
call npx routify
call npm run build
if errorlevel 1 exit /b 1

echo [2/4] Building jarvis-app...
cd /d "%ROOT%"
cargo build --release -p jarvis-app
if errorlevel 1 exit /b 1

echo [3/4] Post-build copy...
python "%ROOT%scripts\tools\post_build.py" --force
if errorlevel 1 exit /b 1

echo [4/4] Building jarvis-gui...
cargo build --release -p jarvis-gui
if errorlevel 1 exit /b 1

python "%ROOT%scripts\tools\post_build.py" --force
echo BUILD OK
exit /b 0
