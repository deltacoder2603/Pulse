use std::collections::HashMap;
use std::process::Command;
use sysinfo::{System, Networks, Disks, Components};

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub memory_mb: u64,
}

fn hr(title: &str) {
    println!("\n┌{:─<78}┐", format!(" {} ", title));
}

fn footer() {
    println!("└{:─<78}┘", "");
}

fn meter(percent: f32, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f32).round() as usize;
    let empty = width - filled;
    format!(
        "{}{} {:>5.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percent
    )
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        format!("{:<width$}", s, width = max)
    } else {
        let t: String = s.chars().take(max - 1).collect();
        format!("{}…", t)
    }
}

/* ---------- Temperature helpers ---------- */

fn fmt_temp(t: Option<f32>) -> String {
    match t {
        Some(v) => format!("{:>5.1}°", v),
        None => "  N/A".to_string(),
    }
}

fn temp_status(t: Option<f32>) -> &'static str {
    match t {
        Some(v) if v < 60.0 => "OK",
        Some(v) if v < 80.0 => "WARM",
        Some(_) => "HOT",
        None => "N/A",
    }
}

/* ---------- CPU via ps ---------- */

fn get_cpu_usage_from_ps() -> HashMap<i32, f32> {
    let output = Command::new("ps")
        .args(["-axo", "pid,pcpu"])
        .output()
        .expect("failed to run ps");

    let text = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(pid), Ok(cpu)) =
                (parts[0].parse::<i32>(), parts[1].parse::<f32>())
            {
                map.insert(pid, cpu);
            }
        }
    }
    map
}

pub fn get_top_processes(sys: &System) -> Vec<ProcessInfo> {
    let cpu_map = get_cpu_usage_from_ps();

    let mut processes: Vec<ProcessInfo> = sys.processes()
        .iter()
        .map(|(pid, p)| {
            let pid_i32 = pid.as_u32() as i32;
            let cpu = cpu_map.get(&pid_i32).copied().unwrap_or(0.0);

            ProcessInfo {
                pid: pid_i32,
                name: p.name().to_string_lossy().to_string(),
                cpu,
                memory_mb: p.memory() / 1024 / 1024,
            }
        })
        .collect();

    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    processes.truncate(10);

    processes
}

/* ---------- Main dashboard ---------- */

pub fn stats() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut networks = Networks::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();

    println!("\n██████╗ ██╗   ██╗██╗     ███████╗███████╗");
    println!("██╔══██╗██║   ██║██║     ██╔════╝██╔════╝");
    println!("██████╔╝██║   ██║██║     ███████╗█████╗  ");
    println!("██╔═══╝ ██║   ██║██║     ╚════██║██╔══╝  ");
    println!("██║     ╚██████╔╝███████╗███████║███████╗");
    println!("╚═╝      ╚═════╝ ╚══════╝╚══════╝╚══════╝");
    println!("              System Monitor\n");

    /* SYSTEM */
    hr("SYSTEM");
    println!("│ Host        │ {:<55} │", System::host_name().unwrap_or("Unknown".into()));
    println!("│ OS          │ {:<55} │", System::os_version().unwrap_or("Unknown".into()));
    println!("│ Kernel      │ {:<55} │", System::kernel_version().unwrap_or("Unknown".into()));
    println!("│ Uptime      │ {:<55} │", format!("{} seconds", System::uptime()));
    footer();

    /* CPU */
    hr("CPU");
    println!("│ {:<8} │ {:<52} │", "Core", "Usage");
    println!("├──────────┼────────────────────────────────────────────────────┤");

    for cpu in sys.cpus() {
        println!(
            "│ {:<8} │ {:<52} │",
            cpu.name(),
            meter(cpu.cpu_usage(), 30)
        );
    }

    println!(
        "│ {:<8} │ {:<52} │",
        "TOTAL",
        meter(sys.global_cpu_usage(), 30)
    );
    footer();

    /* MEMORY */
    hr("MEMORY");
    println!("│ Total      │ {:>10.2} GB │", sys.total_memory() as f64 / 1024.0 / 1024.0);
    println!("│ Used       │ {:>10.2} GB │", sys.used_memory() as f64 / 1024.0 / 1024.0);
    println!("│ Available  │ {:>10.2} GB │", sys.available_memory() as f64 / 1024.0 / 1024.0);
    println!("│ Swap Used  │ {:>10.2} GB │", sys.used_swap() as f64 / 1024.0 / 1024.0);
    footer();

    /* DISKS */
    hr("DISKS");
    println!("│ {:<12} │ {:<12} │ {:>8} │ {:>8} │", "Name", "Mount", "Total", "Free");
    println!("├────────────┼────────────┼──────────┼──────────┤");

    for d in disks.iter() {
        println!(
            "│ {:<12} │ {:<12} │ {:>6}GB │ {:>6}GB │",
            d.name().to_string_lossy(),
            d.mount_point().display(),
            d.total_space() / 1_073_741_824,
            d.available_space() / 1_073_741_824
        );
    }
    footer();

    /* NETWORK */
    hr("NETWORK");
    networks.refresh(false);
    for (iface, data) in networks.iter() {
        println!(
            "│ {:<10} RX {:>6} MB │ TX {:>6} MB │",
            iface,
            data.received() / 1_048_576,
            data.transmitted() / 1_048_576
        );
    }
    footer();

    /* TEMPERATURE */
    hr("TEMPERATURE");
    components.refresh(false);

    if components.is_empty() {
        println!("│ No temperature sensors available on this system │");
    } else {
        println!(
            "│ {:<28} │ {:>6} │ {:>6} │ {:<6} │",
            "Sensor", "Temp", "Max", "State"
        );
        println!("├────────────────────────────┼────────┼────────┼────────┤");

        for c in components.iter() {
            println!(
                "│ {:<28} │ {} │ {} │ {:<6} │",
                truncate(c.label(), 28),
                fmt_temp(c.temperature()),
                fmt_temp(c.max()),
                temp_status(c.temperature())
            );
        }
    }
    footer();

    /* PROCESSES */
    hr("TOP PROCESSES (CPU)");
    println!(
        "│ {:>7} │ {:<28} │ {:>7} │ {:>7} │",
        "PID", "Process", "CPU%", "MEM"
    );
    println!("├─────────┼────────────────────────────┼─────────┼─────────┤");

    let top_processes = get_top_processes(&sys);

    for p in &top_processes {
        println!(
            "│ {:>7} │ {} │ {:>6.1}% │ {:>6}MB │",
            p.pid,
            truncate(&p.name, 28),
            p.cpu,
            p.memory_mb
        );
    }
    footer();
}