mod platform;
mod utils;

pub enum Platform {
    Windows,
    X11,
    Wayland,
    MacOS,
}

pub fn capture(platform: Platform) {
    match platform {
        Platform::Windows => todo!("Implement Windows capture."),
        Platform::X11 => todo!("Implement X11 capture."),
        Platform::Wayland => todo!("Implement Wayland capture."),
        Platform::MacOS => todo!("Implement MacOS capture."),
    }
}
