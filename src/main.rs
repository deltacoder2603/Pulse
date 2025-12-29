mod dashboard;
mod analyze;

use sysinfo::System;

fn main() {
    println!("Welcome to the Pulse");

    let mut sys = System::new_all();
    sys.refresh_all();

    // Print full dashboard UI
    dashboard::stats();

    // Get structured process data (REAL source of truth)
    let top_processes = dashboard::get_top_processes(&sys);

    // Analyze processes (logic, not UI parsing)
    analyze::analyze_processes(&top_processes);
}