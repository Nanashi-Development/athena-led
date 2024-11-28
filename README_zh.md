# Athena LED 控制器

[原项目](https://github.com/NONGFAH/athena-led) 的 Rust 实现，用于在 OpenWrt 设备上控制 LED 点阵显示屏。

[English](README.md) | 简体中文

## 功能特性

- 显示当前时间和日期
- 显示系统温度
- 自定义文本显示
- 可调节亮度级别
- 多种显示模式
- HTTP 状态监控

## 构建说明

1. 确保已安装 Rust 和 Cargo
2. 针对 OpenWrt（通常为 ARM 架构）设置交叉编译环境
3. 构建项目：
   ```bash
   cargo build --release
   ```
4. 编译后的二进制文件位于 `target/release/athena-led`

## 交叉编译

本项目支持使用 Docker/Podman 进行 OpenWrt 设备的交叉编译。目标架构为 `aarch64-unknown-linux-musl`。

### 前置要求

- 安装 Docker 或 Podman
- 基本了解容器操作

### 构建步骤

1. 构建容器镜像：
```bash
podman build -t athena-led-builder .
```

2. 运行容器并构建项目：
```bash
# 运行容器
podman run -d --name athena-led-build athena-led-builder

# 复制编译好的二进制文件
podman cp athena-led-build:/home/rust/release/athena-led ./output/aarch64-unknown-linux-musl/

# 清理容器
podman rm -f athena-led-build
```

编译好的二进制文件将位于 `output/aarch64-unknown-linux-musl/athena-led`。

### 技术细节

- 目标架构：`aarch64-unknown-linux-musl`
- 运行时库：musl（更好地兼容 OpenWrt）
- 静态链接：所有依赖都静态链接
- 工具链：
  - 交叉编译器：gcc-aarch64-linux-gnu
  - 环境变量：
    ```
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
    CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc
    CXX_aarch64_unknown_linux_musl=aarch64-linux-gnu-g++
    ```

## 安装说明

将编译好的二进制文件 `athena-led` 复制到 OpenWrt 设备的 `/usr/sbin/` 目录下。

## 使用方法

```bash
athena-led [选项]

选项说明：
    --status <状态>            设置状态字符串 [默认: ""]
    --seconds <秒数>           更新间隔（秒） [默认: 5]
    --light-level <亮度>       设置亮度级别（0-255） [默认: 5]
    --option <选项>            显示模式（如 "date"、"timeBlink"） [默认: "date timeBlink"]
    --value <值>              自定义显示字符 [默认: "abcdefghijklmnopqrstuvwxyz0123456789+-*/=.:：℃"]
    --url <URL>               状态监控的 URL [默认: "https://www.baidu.com/"]
```

## 开源许可

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。
