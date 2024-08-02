//! # time_summary
//!
//! Prints out the time summary for the flow.
//!

use std::cmp::Ordering;
use std::time::SystemTime;

use crate::types::{CliArgs, Config};

pub(crate) fn is_time_summary_enabled() -> bool {
    envmnt::is("CARGO_MAKE_PRINT_TIME_SUMMARY")
}

pub(crate) fn add(time_summary: &mut Vec<(String, u128)>, name: &str, start_time: SystemTime) {
    match start_time.elapsed() {
        Ok(elapsed) => time_summary.push((name.to_string(), elapsed.as_millis())),
        _ => (),
    };
}

pub(crate) fn print(time_summary: &Vec<(String, u128)>) {
    if is_time_summary_enabled() {
        let mut time_summary_sorted = time_summary.clone();
        time_summary_sorted
            .sort_by(|entry1, entry2| entry2.1.partial_cmp(&entry1.1).unwrap_or(Ordering::Equal));

        let mut total_time = 0;
        let mut max_name_size = 0;
        for entry in &time_summary_sorted {
            total_time = total_time + entry.1;
            let name_size = entry.0.len();
            if max_name_size < name_size {
                max_name_size = name_size;
            }
        }

        info!("==================Time Summary==================");
        for entry in &time_summary_sorted {
            let percentage = (entry.1 as f64 / total_time as f64) * 100.0;
            let seconds = entry.1 as f64 / 1000.0;

            let percentage_size = if percentage >= 100.0 {
                3
            } else if percentage >= 10.0 {
                2
            } else {
                1
            };
            let mut gap_size = 4 - percentage_size;
            let value_gap = format!("{: <1$}", "", gap_size);

            let name_size = entry.0.len();
            gap_size = max_name_size - name_size + 3;
            let name_gap = format!("{: <1$}", "", gap_size);

            info!(
                "{}:{}{:.2}%{}   {:.2} seconds",
                entry.0, name_gap, percentage, value_gap, seconds
            );
        }
        info!("================================================");
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
