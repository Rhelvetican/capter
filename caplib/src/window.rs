//! This module provides functions to interact with windows on the system.

use anyhow::Result;
use xcap::{
    image::{ImageBuffer, Rgba},
    Window,
};

/// Get all windows on the system.
pub fn get_windows() -> Result<Vec<Window>> {
    match Window::all() {
        Ok(windows) => Ok(windows),
        Err(e) => Err(e.into()),
    }
}

/// Screen capture the window.
pub fn screenshot_window(window: Window) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = window.capture_image()?;
    Ok(image)
}
