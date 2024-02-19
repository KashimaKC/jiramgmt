
use std::time::Duration;

use sysinfo::{ CpuRefreshKind, RefreshKind, System };
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct CpuPayload {
    value: f32,
}


#[tauri::command]
pub async fn get_cpu_usage(app_window: tauri::Window) {

    std::thread::spawn(move || {

        let window = app_window.get_window("main").unwrap();
        let running: bool = true;
        let mut s = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );

        while running {
            std::thread::sleep(Duration::from_secs(1));
            s.refresh_cpu();

            let mut sum = 0.0;

            for cpu in s.cpus() {
                sum = sum + cpu.cpu_usage();
            }

            let _ = window.emit("cpu_usage", CpuPayload { value: sum / 8.0 });
        }
    });
    

}
