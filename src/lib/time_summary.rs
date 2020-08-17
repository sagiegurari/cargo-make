//! # time_summary
//!
//! Prints out the time summary for the flow.
//!

use crate::types::{CliArgs, Config};
use envmnt;
use std::time::SystemTime;

pub(crate) fn add(time_summary: &mut Vec<(String, u128)>, name: &str, start_time: SystemTime) {
    match start_time.elapsed() {
        Ok(elapsed) => time_summary.push((name.to_string(), elapsed.as_millis())),
        _ => (),
    };
}

pub(crate) fn print(time_summary: &Vec<(String, u128)>) {
    if envmnt::is("CARGO_MAKE_PRINT_TIME_SUMMARY") {
        let mut total_time = 0;
        let mut max_name_size = 0;
        for entry in time_summary {
            total_time = total_time + entry.1;
            let name_size = entry.0.len();
            if max_name_size < name_size {
                max_name_size = name_size;
            }
        }

        info!("=====Time Summary=====");
        for entry in time_summary {
            let percentage = (entry.1 as f64 / total_time as f64) * 100.0;
            let seconds = entry.1 as f64 / 1000.0;
            let name_size = entry.0.len();
            let gap_size = max_name_size - name_size + 3;
            let gap = format!("{: <1$}", "", gap_size);
            info!(
                "{}:{}{:.2}%\t   {:.2} seconds",
                entry.0, gap, percentage, seconds
            );
        }
    }
}

pub(crate) fn init(config: &Config, cli_args: &CliArgs) {
    if config.config.time_summary.unwrap_or(false)
        || cli_args.print_time_summary
        || envmnt::is("CARGO_MAKE_CI")
    {
        envmnt::set_bool("CARGO_MAKE_PRINT_TIME_SUMMARY", true);
    }
}
