use std::{os::unix::process::CommandExt, thread::sleep, time::Duration};

use clap::{Args, Parser, Subcommand};
use regex::Regex;

fn main() {
    let CLI { command } = CLI::parse();

    match command {
        Command::Get => notify(&get_battery()),

        Command::Watch(Watcher {
            notification_points,
            interval,
        }) => {
            let interval: Duration = interval.into();

            let mut battery_state = get_battery();
            let mut last_percentage = 101;

            loop {
                for &notif_point in notification_points.iter().rev() {
                    if last_percentage > notif_point && battery_state.percentage <= notif_point {
                        last_percentage = battery_state.percentage;
                        notify(&battery_state);
                    }
                }

                sleep(interval);

                battery_state = get_battery();
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn get_battery() -> AcpiOutput {
    let acpi_output = std::process::Command::new("acpi")
        .output()
        .expect("Failed to run 'acpi' command")
        .stdout;

    let acpi_output = String::from_utf8(acpi_output).expect("'acpi' output is not valid utf8");

    let percentage_matcher = Regex::new(r"\d+%").unwrap();
    let expected_time_matcher = Regex::new(r"\d\d:\d\d:\d\d").unwrap();

    let percentage = percentage_matcher
        .find(&acpi_output)
        .expect("Failed to parse acpi output for percentage")
        .as_str()
        .strip_suffix("%")
        .unwrap()
        .parse()
        .expect("'acpi' percentage value too large to parse");

    let charging_state = acpi_output.contains("Charging");

    let expected_time = expected_time_matcher
        .find(&acpi_output)
        .map(|m| m.as_str())
        .unwrap_or("")
        .to_owned();

    AcpiOutput {
        percentage,
        charging_state,
        expected_time,
    }
}

#[cfg(target_os = "linux")]
fn notify(value: &AcpiOutput) {
    let AcpiOutput {
        percentage,
        charging_state: charging,
        expected_time,
    } = value;

    let output = format!(
        "{percentage}%, {expected_time} {}",
        if *charging { "⤒" } else { "⤓" }
    );

    let _ = std::process::Command::new("notify-send")
        .arg(output).spawn();
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct AcpiOutput {
    percentage: usize,
    charging_state: bool,
    expected_time: String,
}

/// Command for handling battery notifications on linux.
/// Depends on the 'acpi' and 'notify-send' utilities.
#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug, Default)]
enum Command {
    /// Sends notification with current battery state
    #[command()]
    #[default]
    Get,

    /// Sends notification when any notification point
    /// is reached
    #[command()]
    Watch(Watcher),
}

#[derive(Args, Clone, Debug)]
struct Watcher {
    /// Notification is sent when battery falls below
    /// any specified level
    notification_points: Vec<usize>,

    /// Time between battery checks
    #[arg(short, long, default_value = "10s")]
    interval: ParsedDuration,
}

#[derive(Clone, Debug)]
struct ParsedDuration {
    value: Duration,
}

impl From<&str> for ParsedDuration {
    fn from(value: &str) -> Self {
        Self {
            value: parse_duration::parse(value).expect("Failed to parse duration"),
        }
    }
}

impl Into<Duration> for ParsedDuration {
    fn into(self) -> Duration {
        self.value
    }
}
