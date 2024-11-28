# Athena LED ${{ env.VERSION }}

## Changes

### ✨ New Features
${{ env.FEAT_CHANGES }}

### 🐛 Bug Fixes
${{ env.FIX_CHANGES }}

### 📝 Documentation
${{ env.DOC_CHANGES }}

### ⚡ Performance
${{ env.PERF_CHANGES }}

### 🔨 Other Changes
${{ env.OTHER_CHANGES }}

## Installation

1. Download the `athena-led-aarch64-musl.tar.gz` file
2. Extract the file:
   ```bash
   tar xzf athena-led-aarch64-musl.tar.gz
   ```
3. Copy the binary to your OpenWrt device:
   ```bash
   scp athena-led root@your-openwrt-device:/usr/sbin/
   ```

## Checksums

```
${{ env.SHA256SUM }}
```

## Build Information

- Target: aarch64-unknown-linux-musl
- Rust Version: ${{ env.RUST_VERSION }}
- Build Date: ${{ env.BUILD_DATE }}

## Support

If you encounter any issues, please report them on [GitHub Issues](https://github.com/NONGFAH/athena-led/issues).
