use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, AppHandle, SystemTrayEvent, Manager};

// 托盘菜单
pub fn menu() -> SystemTray {
    let about_mno = CustomMenuItem::new("about".to_string(), "关于 Chat MNO");
    let show = CustomMenuItem::new("show".to_string(), "显示界面");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(about_mno)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}


// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        },
        SystemTrayEvent::DoubleClick { .. } => {
            window.show().unwrap();
            window.set_focus().unwrap();
        },
        _ => {}
    }
}