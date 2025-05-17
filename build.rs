fn main() {
    slint_build::compile("ui/mainWindow.slint").expect("Slint build failed");
    //Windows specific:
    println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS,10.0");
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "Minecraft Mod Manager");
    res.set("CompanyName", "_1ms");
    res.set("ProductName", "McModManager");
    res.set("LegalCopyright", "Copyright © 2025 _1ms");
    // Use your Cargo version or hardcode a string
    res.set("FileVersion", env!("CARGO_PKG_VERSION"));
    res.set("ProductVersion", env!("CARGO_PKG_VERSION"));

    // 3) Compile and link the resource into the EXE
    res.compile().expect("failed to compile resources");
}
