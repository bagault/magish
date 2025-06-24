@echo off
REM Self-updating Windows installer for MagiSH
setlocal
set REPO=bagault/magish
set ZIP_NAME=magish-windows-x86_64.zip
set DEST_PATH="%ProgramFiles%\MagiSH"
set SHORTCUT="%USERPROFILE%\Desktop\MagiSH.lnk"

REM Relaunch as administrator if not already
openfiles >nul 2>&1
if %errorlevel% neq 0 (
    echo Requesting administrator privileges...
    powershell -Command "Start-Process '%~f0' -Verb runAs"
    exit /b
)

REM Download the latest production release zip from GitHub
set DOWNLOAD_URL=https://github.com/bagault/magish/releases/download/production/%ZIP_NAME%

REM Download the zip file
powershell -Command "Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP%\%ZIP_NAME%'"

REM Extract all files from the zip
powershell -Command "Expand-Archive -Path '%TEMP%\%ZIP_NAME%' -DestinationPath '%TEMP%\magish_extract' -Force"

REM Create destination folder
if not exist %DEST_PATH% mkdir %DEST_PATH%

REM Copy all files to Program Files\MagiSH
copy /Y %TEMP%\magish_extract\* %DEST_PATH%\

REM Create desktop shortcut using PowerShell
powershell -Command "$s=(New-Object -COM WScript.Shell).CreateShortcut('%SHORTCUT%');$s.TargetPath='%DEST_PATH%\magish.exe';$s.IconLocation='%DEST_PATH%\favicon.ico';$s.Save()"

echo Installation complete. Shortcut created on Desktop.
pause
