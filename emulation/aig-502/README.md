# AIG-502 Emulation (Windows, Modbus RTU)

## Prerequisites

1. **com0com** (virtual serial port pairs)
   - Download: https://sourceforge.net/projects/com0com/
   - On Windows 10/11, enable test signing: `bcdedit.exe -set TESTSIGNING ON`, reboot.
   - Open `setupg.exe`, click **Add Pair**, set `COM10` and `COM11`, Apply.

2. **OpenModSim** (Modbus RTU Slave)
   - Download: https://github.com/mgrillo75/OpenModSim
   - Run, create RTU Slave on `COM11`, baud 9600, 8N1, slave ID = 1.
   - Add holding registers 0..10.

## Run concentrator against emulation

```
cargo run -- --api-addr 127.0.0.1:18443 run
```

Check logs: `modbus rtu read` (slave=1, reg=0..9).

## Troubleshooting

- `open COM10: Access denied` -> another app holds COM10.
- `read: timeout` -> OpenModSim not running, or wrong COM11/slave ID.
- `read: illegal data address` -> registers 0..10 not defined in OpenModSim.

## Notes

- Tests **only Modbus RTU layer** (modbus.rs).
- ThingsPro Edge (HTTP API, twin) is NOT emulated. Use mock HTTP server for aig/client.rs (later).
