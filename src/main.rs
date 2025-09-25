use std::os::unix::net::{UnixStream};
use std::env;
use std::io::{BufRead, BufReader};
use std::process::Command;
use serde_json::{Value};

fn get_monitor_names() -> Vec<String> {
    let output = Command::new("hyprctl")
        .args(&["monitors", "all", "-j"])
        .output()
        .expect("Failed to execute hyprctl monitors command");

    if !output.status.success() {
        eprintln!("hyprctl monitors command failed with status: {:?}", output.status);
        return vec![];
    }

    let json = serde_json::from_slice::<Value>(&output.stdout).unwrap();

    let mut names = Vec::new();
    for monitor in json.as_array().unwrap() {
        if let Some(name) = monitor["name"].as_str() {
            names.push(name.to_string());
        }
    }
    return names;
}

fn enable_builtin_monitor() {
    let monitor_names = get_monitor_names();
    if monitor_names.len() == 0 {
        eprintln!("No monitors found, cannot enable built-in monitor.");
        return;
    }
    let mut cmd = Command::new("hyprctl");
    cmd.args(&["keyword", "monitor", format!("{},preferred,auto,auto", monitor_names[0]).as_str()]);
    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to enable monitor: {:?}", status);
            } 
        }
        Err(e) => eprintln!("Failed to execute command: {}", e)
    }
}

fn handle_client(stream: UnixStream) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                if msg.contains("monitorremovedv2>>") {
                    enable_builtin_monitor();
                }
            },
            Err(e) => {
                eprintln!("Error reading: {e}");
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let xdg_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set in the environment");
    let inst_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set in the environment");
    let hyprland_socket_path = format!("{}/hypr/{}/.socket2.sock", xdg_dir, inst_sig);

    println!("Binding to socket path: {}", hyprland_socket_path);
    let listener = UnixStream::connect(hyprland_socket_path);

    match listener {
        Ok(stream) => {
            println!("Successfully connected to Hyprland socket.");
            handle_client(stream);
        },
        Err(e) => {
            eprintln!("Failed to connect to Hyprland socket: {}", e);
        }
    }
    Ok(())
}
