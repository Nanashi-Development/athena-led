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

项目包含了一个处理整个交叉编译过程的构建脚本：

```bash
./scripts/aarch64-unknown-linux-musl-build.sh
```

该脚本会：
1. 创建必要的输出目录
2. 构建包含所有必需依赖的容器镜像
3. 为 aarch64-unknown-linux-musl 目标编译项目
4. 提取编译好的二进制文件

最终的二进制文件将位于 `output/aarch64-unknown-linux-musl/athena-led`。

### 技术细节

- 目标架构：`aarch64-unknown-linux-musl`
- 运行时库：musl（更好地兼容 OpenWrt）
- 静态链接：所有依赖都静态链接
- 二进制大小：约 2.2M（经过大小优化）
- 工具链：
  - 交叉编译器：gcc-aarch64-linux-gnu
  - 环境变量由构建脚本自动设置

## 安装说明

将编译好的二进制文件 `athena-led` 复制到 OpenWrt 设备的 `/usr/sbin/` 目录下。

## 使用方法

```bash
athena-led [选项]

选项说明：
    --status <状态>            设置状态字符串 [默认: ""]
    --seconds <秒数>           更新间隔（秒） [默认: 5]
    --light-level <亮度>       设置亮度级别（0-7） [默认: 5]
    --option <选项>            显示模式（如 "date"、"timeBlink"） [默认: "date timeBlink"]
    --value <值>              自定义显示字符 [默认: "abcdefghijklmnopqrstuvwxyz0123456789+-*/=.:：℃"]
    --url <URL>               状态监控的 URL [默认: "https://www.baidu.com/"]
    --temp-flag <标志>         温度传感器标志（0:nss-top, 1:nss, 2:wcss-phya0, 3:wcss-phya1, 4:cpu, 5:lpass, 6:ddrss） [默认: "4"]
```

## 常见问题

1. **时间显示问题**  
   如果显示的时间与系统时区不匹配，请确保系统已安装所需的时区数据包：
   - OpenWrt 系统：安装 `zoneinfo-core` 和对应地区的包（如 `zoneinfo-asia`）
   - 其他 Linux 发行版：安装 `tzdata` 包

## 开源许可

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。
