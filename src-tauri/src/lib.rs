use serde::Serialize;
use std::time::SystemTime;
use sysinfo::{ProcessesToUpdate, System};
use tauri::Emitter;

const BYTES_TO_MB: u64 = 1024 * 1024;

#[derive(Serialize, Clone)]
struct DataPoint {
    timestamp: f32,
    cpu_usage: f32,
    memory: u64,
    processes: Vec<Process>,
}

#[derive(Serialize, Clone)]
struct Process {
    pid: u32,
    name: String,
    cpu: f32,
    memory: u64,
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let handle = app.handle().clone();

            tauri::async_runtime::spawn_blocking(move || {
                let mut sys = System::new();
                let initial_time = SystemTime::now();
                loop {
                    sys.refresh_cpu_usage();
                    sys.refresh_memory();
                    sys.refresh_processes(ProcessesToUpdate::All, true);

                    let mut processes: Vec<_> = sys
                        .processes()
                        .iter()
                        .map(|(pid, proc)| Process {
                            pid: pid.as_u32(),
                            name: proc.name().to_string_lossy().to_string(),
                            cpu: proc.cpu_usage() * 100.0,
                            memory: proc.memory() / BYTES_TO_MB,
                        })
                        .collect();

                    processes.sort_by_key(|p| p.name.clone());

                    let data_point = DataPoint {
                        timestamp: SystemTime::now()
                            .duration_since(initial_time)
                            .unwrap()
                            .as_secs_f32(),
                        cpu_usage: sys.global_cpu_usage(),
                        memory: sys.used_memory() / BYTES_TO_MB,
                        processes,
                    };
                    handle.emit("data_point", data_point).unwrap();

                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn kill_process(pid: u32) {
    let sys = System::new_all();
    if let Some(proc) = sys.process(sysinfo::Pid::from_u32(pid)) {
        proc.kill();
    }
}
