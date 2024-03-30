
# 资料
[其他 Rust 嵌入式 HAL 列表]: https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates
[可以使用这个 quickstart 快速启动一个项目]: https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart/
[芯片手册](https://www.st.com/resource/en/datasheet/lsm303agr.pdf)
[关于开发板的更多细节](https://tech.microbit.org/hardware/).

# 工具安装

```shell
rustup component add llvm-tools-preview
cargo install cargo-binutils --vers 0.3.3

# This will install the probe-rs, cargo-flash and cargo-embed
cargo install probe-rs --features cli

# for arch linux
sudo pacman -S \
  arm-none-eabi-gdb \
  minicom
```

## udev 规则配置
首先检索你的设备
```shell
lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 007: ID 0d28:0204 NXP ARM mbed
```
然后在该路径下 `/etc/udev/rules.d` 创建规则文件 `99-microbit.rules`:
```text
# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```
然后执行该命令进行刷新:
```shell
sudo udevadm control --reload-rules
```
如果你的设备已经连接你的电脑, 重新拔插设备.

## 检查设备的权限
```shell
lsusb | grep -i "NXP ARM mbed"
Bus 001 Device 007: ID 0d28:0204 NXP ARM mbed
```
``` console
$ ls -l /dev/bus/usb/001/007
crw-rw-rw- 1 root root 189, 6 Mar28日 20:38 /dev/bus/usb/001/007
```
权限应该是 `crw-rw-rw-` 否则就再检查下 `/etc/udev/rules.d/99-microbit.rules` 规则文件
并且进行执行
```shell
sudo udevadm control --reload-rules
```
然后重新拔插设备

# 编译执行
```shell
# 增加所需的编译类型
rustup target add thumbv7em-none-eabihf
cargo embed
```

# 其他命令
```shell
# 检查可执行文件的文件头
cargo readobj  -- --file-headers
```

需要 gbd debug 需要修改文件 Embed.toml
```toml
[default.reset]
halt_afterwards = true

[default.rtt]
enabled = false

[default.gdb]
enabled = true
```
然后执行
```shell
# debug
arm-none-eabi-gdb <实际产物> 
# gdb 中执行 target remote localhost:1337
```

需要 rtt 需要修改文件 Embed.toml
```toml
[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```
