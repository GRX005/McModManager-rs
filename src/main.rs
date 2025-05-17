#![windows_subsystem = "windows"]

use slint::BackendSelector;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> { //TODO error log.
    BackendSelector::new().require_opengl().select()?;
    let main_window = MainWindow::new()?;
    main_window.run()
}