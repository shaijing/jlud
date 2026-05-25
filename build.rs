fn main() {
    if cfg!(target_os = "windows") {
        tauri_winres::WindowsResource::new()
            .set_icon("assets/icon/jlud.ico")
            .compile()
            .expect("Failed to compile Windows resources");
    }
}
