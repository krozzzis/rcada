# rcada_modbus_simulator

Simple Modbus-TCP simulator with time-based sensor value simulation.

## Running

```bash
cargo run -- 127.0.0.1:502
```

Default port is 502.

## Register Layout

| Address | Function | Description | Range |
|---------|----------|-------------|-------|
| 0 | FC 03/04 | Temperature | 150-250 (15.0-25.0C) |
| 1 | FC 03/04 | Humidity | 400-600 (40.0-60.0%) |
| 2 | FC 03/04 | Pressure | 1003-1023 (1003-1023hPa) |
| 3 | FC 03/04 | Voltage | 100-140 (10.0-14.0V) |
| 4 | FC 03/04 | Current | 800-1200 (8.00-12.00A) |
| 5 | FC 03/04 | Status | 1 |

## Building

```bash
cargo build --release
```
