# Athena LED

English | [简体中文](README_zh.md)

A Rust port of [athena-led](https://github.com/NONGFAH/athena-led) for controlling LED matrix displays on OpenWrt devices.

## Features

- Display current time and date
- Show system temperature
- Custom text display
- Adjustable brightness levels
- Multiple display modes
- HTTP status monitoring

## Building

1. Make sure you have Rust and Cargo installed
2. For cross-compilation to OpenWrt (typically ARM architecture), set up your cross-compilation environment
3. Build the project:

   ```bash
   cargo build --release
   ```

4. The compiled binary will be in `target/release/athena-led`

## Cross Compilation

This project supports cross-compilation for OpenWrt devices using Docker/Podman. The target architecture is `aarch64-unknown-linux-musl`.

### Prerequisites

- Docker or Podman installed
- Basic understanding of container operations

### Build Steps

The project includes a build script that handles the entire cross-compilation process:

```bash
./scripts/aarch64-unknown-linux-musl-build.sh
```

The script will:

1. Create necessary output directories
2. Build the container image with all required dependencies
3. Compile the project for aarch64-unknown-linux-musl target
4. Extract the compiled binary

The final binary will be available at `output/aarch64-unknown-linux-musl/athena-led`.

### Technical Details

- Target: `aarch64-unknown-linux-musl`
- Libc: musl (for better compatibility with OpenWrt)
- Static linking: All dependencies are statically linked
- Binary size: ~2.2M (optimized for size)
- Toolchain:
  - Cross compiler: gcc-aarch64-linux-gnu
  - Environment variables are automatically set by the build script

## Installation

Copy the compiled binary `athena-led` to your OpenWrt device's `/usr/sbin/` directory.

## Usage

```bash
athena-led [OPTIONS]

Options:
    --status <STATUS>          Set status string [default: ""]
    --seconds <SECONDS>        Update interval in seconds [default: 5]
    --light-level <LEVEL>      Set brightness level (0-7) [default: 5]
    --option <OPTION>          Display mode (e.g., "date", "timeBlink") [default: "date timeBlink"]
    --value <VALUE>           Custom display characters [default: "abcdefghijklmnopqrstuvwxyz0123456789+-*/=.:：℃"]
    --url <URL>               URL for status monitoring [default: "https://www.baidu.com/"]
    --temp-flag <FLAG>        Temperature sensor flag (0:nss-top, 1:nss, 2:wcss-phya0, 3:wcss-phya1, 4:cpu, 5:lpass, 6:ddrss) [default: "4"]
```

## FAQ

1. **Time Display Issues**  
   If the displayed time doesn't match your system timezone, please ensure the required timezone data packages are installed on your system:
   - For OpenWrt: Install `zoneinfo-core` and your region's package (e.g., `zoneinfo-asia`)
   - For other Linux distributions: Install `tzdata` package

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
