@echo off
set "ROOT=%~dp0..\..\"
cd /d "%ROOT%"
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
cargo build -p jarvis-app
python "%ROOT%scripts\tools\post_build.py" --force
