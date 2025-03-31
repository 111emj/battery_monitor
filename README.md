# Dependencies

Currently supports linux operating systems only.

Depends on `acpi` and `notify-send` commands.

## Installation

### Arch Linux
`pacman -S --needed acpi libnotify`

# Build from Source

## Build Dependencies

Rust Language: https://www.rust-lang.org/tools/install

## Commands

```
git clone https://github.com/111emj/battery_monitor.git
cd battery_monitor
cargo build --release
```

Binary will be placed at target/release/battery\_monitor
