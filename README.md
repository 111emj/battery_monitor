# Example Usage

To send a notification with current battery state:

```battery_monitor get```

To start a watcher that sends notifications at 50%, 20%, and 10%:

```battery_monitor watch 50 20 10```

# Dependencies

Currently supports linux operating systems only.

Depends on `acpi` and `notify-send` commands.

## Dependency Installation

### Arch Linux
`pacman -S --needed acpi libnotify`

# Build from Source

## Build Dependencies

Rust Language: https://www.rust-lang.org/tools/install

## Build Commands

BASH:

```
git clone https://github.com/111emj/battery_monitor.git
cd battery_monitor
cargo build --release
```

Binary will be placed at `target/release/battery_monitor`
