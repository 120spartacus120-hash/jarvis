@echo off
setlocal
set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"

call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if errorlevel 1 exit /b 1

python "%ROOT%scripts\tools\post_build.py" --force
cargo build --release -p jarvis-app
if errorlevel 1 exit /b 1
python "%ROOT%scripts\tools\post_build.py" --force
cargo build --release -p jarvis-gui
if errorlevel 1 exit /b 1
python "%ROOT%scripts\tools\post_build.py" --force
echo REBUILD OK
exit /b 0
