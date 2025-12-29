use crate::dashboard::ProcessInfo;
use std::io::{self, Write};
use std::process::Command;

/// Analyze processes and warn about high resource usage
pub fn analyze_processes(processes: &[ProcessInfo]) {
    let mut heavy = Vec::new();

    for p in processes {
        if p.cpu >= 20.0 || p.memory_mb >= 500 {
            heavy.push(p);
        }
    }

    if heavy.is_empty() {
        println!("\n✅ No high resource consuming processes detected.");
        return;
    }

    println!("\n⚠️ High resource consuming processes detected:\n");

    for p in &heavy {
        println!(
            "• PID {} ({}) → CPU: {:.1}% | MEM: {} MB",
            p.pid, p.name, p.cpu, p.memory_mb
        );
    }

    // Ask user if they want to terminate a process
    print!("\nDo you want to terminate any process? (y/n): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim().eq_ignore_ascii_case("y") {
        ask_and_kill();
    } else {
        println!("✔ No processes terminated.");
    }
}

/// Ask for PID and terminate it
fn ask_and_kill() {
    print!("Enter PID to terminate: ");
    io::stdout().flush().unwrap();

    let mut pid_input = String::new();
    io::stdin().read_line(&mut pid_input).unwrap();

    let pid: i32 = match pid_input.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            println!("❌ Invalid PID");
            return;
        }
    };

    match kill_process(pid) {
        Ok(_) => println!("✅ Process {pid} terminated successfully."),
        Err(e) => println!("❌ Failed to kill process: {e}"),
    }
}

/// Cross-platform process termination
fn kill_process(pid: i32) -> Result<(), String> {
    #[cfg(target_family = "unix")]
    {
        Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .status()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_family = "windows")]
    {
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}