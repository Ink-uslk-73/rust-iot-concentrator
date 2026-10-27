$ErrorActionPreference = "Stop"
$Root = "C:\rust-iot-concentrator"
Write-Host "=== Emulation files check ===" -ForegroundColor Cyan
$config = "$Root\emulation\aig-502\config.json"
$regs = "$Root\emulation\aig-502\registers.json"
if (Test-Path $config) { Write-Host "[OK] config.json" -ForegroundColor Green; Get-Content $config | ConvertFrom-Json | Out-Null } else { Write-Host "[MISSING] config.json" -ForegroundColor Red }
if (Test-Path $regs) { Write-Host "[OK] registers.json" -ForegroundColor Green; Get-Content $regs | ConvertFrom-Json | Out-Null } else { Write-Host "[MISSING] registers.json" -ForegroundColor Red }
