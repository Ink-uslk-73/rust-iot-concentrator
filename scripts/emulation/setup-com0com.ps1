Write-Host "=== AIG-502 Emulation Setup ===" -ForegroundColor Cyan
Write-Host "1. Install com0com (as Administrator):" -ForegroundColor Yellow
Write-Host "   https://sourceforge.net/projects/com0com/"
Write-Host "   On Windows 10/11: bcdedit.exe -set TESTSIGNING ON, reboot."
Write-Host "2. Create pair COM10 <-> COM11 via setupg.exe."
Write-Host "3. Run OpenModSim on COM11 (slave ID = 1, holding registers 0..10)."
Write-Host "4. Run: cargo run -- --api-addr 127.0.0.1:18443 run"
Write-Host "5. Check logs: modbus rtu read (slave=1, reg=0..9)"
