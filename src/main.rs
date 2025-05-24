//#![windows_subsystem = "windows"]

use i_slint_backend_winit::WinitWindowAccessor;
use slint::BackendSelector;
use window_vibrancy::apply_mica;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> { //TODO error to log file.
    BackendSelector::new().require_opengl().select()?;
    let mw = MainWindow::new()?;
    mw.window().with_winit_window(|win| {
        apply_mica(win, None).expect("Non-Windows platform.");
    });

    mw.run()
}

