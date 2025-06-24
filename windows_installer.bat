@echo off
REM Self-updating Windows installer for MagiSH
setlocal
set REPO=bagault/magish
set EXE_NAME=magish.exe
set ZIP_NAME=magish-windows-x86_64.zip
set DEST_PATH="%ProgramFiles%\MagiSH"
set SHORTCUT="%USERPROFILE%\Desktop\MagiSH.lnk"

REM Download the latest release zip from GitHub
for /f "tokens=1 delims=" %%A in ('powershell -Command "(Invoke-WebRequest -UseBasicParsing https://api.github.com/repos/%REPO%/releases/latest | ConvertFrom-Json).assets | Where-Object { $_.name -eq '%ZIP_NAME%' } | Select-Object -ExpandProperty browser_download_url"') do set DOWNLOAD_URL=%%A

if "%DOWNLOAD_URL%"=="" (
    echo Could not find the latest release download URL.
    pause
    exit /b 1
)

REM Download the zip file
powershell -Command "Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP%\%ZIP_NAME%'"

REM Extract magish.exe from the zip
powershell -Command "Expand-Archive -Path '%TEMP%\%ZIP_NAME%' -DestinationPath '%TEMP%\magish_extract' -Force"

REM Create destination folder
if not exist %DEST_PATH% mkdir %DEST_PATH%

REM Copy executable
copy /Y %TEMP%\magish_extract\%EXE_NAME% %DEST_PATH%\

REM Create desktop shortcut using PowerShell
powershell -Command "$s=(New-Object -COM WScript.Shell).CreateShortcut('%SHORTCUT%');$s.TargetPath='%DEST_PATH%\%EXE_NAME%';$s.IconLocation='%DEST_PATH%\favicon.ico';$s.Save()"

echo Installation complete. Shortcut created on Desktop.
pause
