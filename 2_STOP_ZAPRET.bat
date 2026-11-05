@echo off
title Stop Zapret [@jungle_dev]
chcp 65001 > nul

net session >nul 2>&1
if %errorlevel% neq 0 (
    powershell -NoProfile -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

taskkill /f /im winws.exe >nul 2>&1
net stop WinDivert >nul 2>&1
net stop WinDivert14 >nul 2>&1

echo [*] zapret stopped.
timeout /t 2 >nul
