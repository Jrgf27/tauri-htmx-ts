// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};
use tauri::command;

#[command]
async fn run_python_sidecar(
    projectname: &str,
    reponame: &str,
    authorname: &str,
    email: &str,
    opensourcelicense: &str,
) -> Result<String, String> {
    println!("Starting Sidecar execution");
    let (mut rx, mut _child) = Command::new_sidecar("test")
        .expect("failed to create `test` binary command")
        .args([
            "--project_name",
            projectname,
            "--repo_name",
            reponame,
            "--author_name",
            authorname,
            "--email",
            email,
            "--open_source_license",
            opensourcelicense,
        ])
        .spawn()
        .expect("Failed to spawn sidecar");

    let mut output = String::new();
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            output.push_str(&line);
        }
    }

    Ok(output)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_python_sidecar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
