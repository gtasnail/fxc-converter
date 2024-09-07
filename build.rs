fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./src/logo.ico"); 
        res.compile().expect("Failed to compile resources.");
    }
}
