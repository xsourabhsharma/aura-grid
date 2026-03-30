@echo off
title Aura-Grid: Distributed DeFi Security Suite
color 0B

echo ========================================================
echo   AURA-GRID: High-Performance DeFi Intelligence
echo ========================================================
echo.

echo [Step 1] Building/Checking Rust Core...
cd core
maturin develop --release >nul 2>&1
if %errorlevel% neq 0 (
    echo [!] Warning: Rust core build failed. Falling back to Python scanner.
) else (
    echo [+] Rust Core: ENGAGED.
)
cd ..

echo.
echo [Step 2] Installing Python dependencies...
pip install -r requirements.txt -q
echo [+] Python Packages: INSTALLED.

echo.
echo [Step 3] Starting WebSocket Nerve Center...
start "Aura-Grid: WebSocket Server" cmd /c "python server.py"

echo.
echo [Step 4] Launching API Orchestrator (Node.js)...
cd api
start "Aura-Grid: API" cmd /c "npm run dev"
cd ..

echo.
echo [Step 5] Launching the Live Cyberpunk Dashboard...
timeout /t 2 /nobreak >nul
start dashboard.html

echo.
echo [Step 6] Engaging the Elite Worker Node (AI Scanner)...
timeout /t 2 /nobreak >nul
python worker.py

echo.
echo ========================================================
echo   SYSTEM ONLINE. Monitoring DeFi Grid for Threats...
echo ========================================================
pause >nul
