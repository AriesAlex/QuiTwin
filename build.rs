fn main() {
    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource
            .set_icon("assets/quitwin.ico")
            .set("ProductName", "QuiTwin")
            .set("FileDescription", "QuiTwin Discord and Equicord launcher")
            .set("LegalCopyright", "Copyright (c) 2026 QuiTwin contributors");
        resource
            .compile()
            .expect("failed to compile Windows resources");
    }
}
