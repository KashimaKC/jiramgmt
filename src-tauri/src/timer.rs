use std::time::Duration;
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct CounterPayload {
    value: i32,
}

#[tauri::command]
pub async fn counter(app_window: tauri::Window) {

    std::thread::spawn(move || {
        
        let mut i: i32 = 0;
        let window = app_window.get_window("main").unwrap();
        let running: bool = true;
    
        while running {
            i = i + 1;
            std::thread::sleep(Duration::from_secs(1));
            let _ = window.emit("count", CounterPayload { value: i });
        }   
        
    });
}

