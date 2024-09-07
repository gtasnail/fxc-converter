fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./src/logo.ico"); // This should be the path to your icon
        res.compile().expect("Failed to compile resources.");
    }
}