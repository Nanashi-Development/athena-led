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

### Building

1. Build the container image:
```bash
podman build -t athena-led-builder .
```

2. Run the container and build the project:
```bash
# Run container
podman run -d --name athena-led-build athena-led-builder

# Copy the compiled binary
podman cp athena-led-build:/home/rust/release/athena-led ./output/aarch64-unknown-linux-musl/

# Clean up
podman rm -f athena-led-build
```

The compiled binary will be available in `output/aarch64-unknown-linux-musl/athena-led`.

### Technical Details

- Target: `aarch64-unknown-linux-musl`
- Libc: musl (for better compatibility with OpenWrt)
- Static linking: All dependencies are statically linked
- Toolchain:
  - Cross compiler: gcc-aarch64-linux-gnu
  - Environment variables:
    ```
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
    CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc
    CXX_aarch64_unknown_linux_musl=aarch64-linux-gnu-g++
    ```

## Installation

Copy the compiled binary `athena-led` to your OpenWrt device's `/usr/sbin/` directory.

## Usage

```bash
athena-led [OPTIONS]

Options:
    --status <STATUS>          Set status string [default: ""]
    --seconds <SECONDS>        Update interval in seconds [default: 5]
    --light-level <LEVEL>      Set brightness level (0-255) [default: 5]
    --option <OPTION>          Display mode (e.g., "date", "timeBlink") [default: "date timeBlink"]
    --value <VALUE>           Custom display characters [default: "abcdefghijklmnopqrstuvwxyz0123456789+-*/=.:：℃"]
    --url <URL>               URL for status monitoring [default: "https://www.baidu.com/"]
```

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
