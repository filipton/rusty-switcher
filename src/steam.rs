use anyhow::Result;
use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, SystemExt};

use crate::config::insert_home_dir;

const REGISTRY_PATH: &str = "~/.steam/registry.vdf";

pub fn launch_steam(steam_command: &str) -> Result<()> {
    Command::new("bash")
        .arg("-c")
        .arg(format!("{} /dev/null 2>&1 &", steam_command))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    Ok(())
}

pub fn modify_registry_file(username: String) -> Result<()> {
    let registry_file: String = std::fs::read_to_string(insert_home_dir(REGISTRY_PATH)?)
        .expect("Failed to read registry file");

    let mut tmp_registry_file: Vec<String> = vec![];
    for line in registry_file.lines() {
        if line.contains("AutoLoginUser") {
            let tabs_count = line.matches('\t').count() - 2;

            let auto_login_user: String = format!(
                "{}{}\t\t\"{}\"",
                "\t".repeat(tabs_count),
                "\"AutoLoginUser\"",
                username
            );

            let remember_password: String = format!(
                "{}{}\t\t\"{}\"",
                "\t".repeat(tabs_count),
                "\"RememberPassword\"",
                "1"
            );

            tmp_registry_file.push(auto_login_user);
            tmp_registry_file.push(remember_password);

            continue;
        } else if line.contains("RememberPassword") {
            continue;
        }

        tmp_registry_file.push(line.to_string());
    }

    let output_file: String = tmp_registry_file.join("\n");
    std::fs::write(insert_home_dir(REGISTRY_PATH)?, output_file)?;

    Ok(())
}


pub fn kill_steam() {
    let mut system = sysinfo::System::new();
    system.refresh_all();

    for (pid, process) in system.processes() {
        if process.name().contains("steam") {
            println!("Killing: [{}] {}", pid, process.name());
            process.kill();
        }
    }
}
