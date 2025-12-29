# Pulse

A terminal-based system monitor written in Rust that provides real-time insights into your system's performance, resource usage, and running processes.

## Features

- **System Information**: Display hostname, OS version, kernel version, and system uptime
- **CPU Monitoring**: Per-core CPU usage with visual meters
- **Memory Statistics**: Total, used, available memory and swap usage
- **Disk Usage**: Monitor disk space across all mounted filesystems
- **Network Statistics**: Track network interface receive/transmit data
- **Temperature Monitoring**: Display component temperatures with status indicators
- **Process Analysis**: View top processes by CPU usage and identify resource-heavy processes
- **Process Management**: Option to terminate high-resource-consuming processes

## Requirements

- Rust (latest stable version recommended)
- Unix-like system (macOS, Linux) or Windows

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd pulse
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

Or run the release binary:
```bash
./target/release/pulse
```

## Usage

Simply run the application to see a comprehensive dashboard of your system's status:

```bash
cargo run
```

The dashboard displays:
- System information
- CPU usage per core
- Memory statistics
- Disk usage
- Network activity
- Temperature readings
- Top 10 processes by CPU usage

### Process Analysis

After displaying the dashboard, Pulse automatically analyzes running processes and will:
- Warn you about processes consuming more than 20% CPU or 500MB of memory
- Optionally allow you to terminate problematic processes

When high-resource processes are detected, you'll be prompted:
```
Do you want to terminate any process? (y/n):
```

If you choose 'y', you can enter a PID to terminate that process.

## Project Structure

```
pulse/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # Entry point and orchestration
│   ├── dashboard.rs    # System statistics display and process listing
│   └── analyze.rs      # Process analysis and termination logic
└── README.md           # This file
```

## Dependencies

- [sysinfo](https://crates.io/crates/sysinfo) - System information gathering library

## Platform Support

- **Unix-like systems** (macOS, Linux): Full support with `kill` command for process termination
- **Windows**: Supported with `taskkill` command for process termination

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Author

[Add your name/contact information here]

