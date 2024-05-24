//! This module provides functions to interact with windows on the system.

use anyhow::{anyhow, Result};
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

/// Get the window by its name.
pub fn get_window_by_app_name(name: &str) -> Result<Window> {
    let windows = get_windows()?;
    for window in windows {
        if window.app_name() == name {
            return Ok(window);
        }
    }
    Err(anyhow!("Window not found"))
}

/// Get the window by its title.
pub fn get_window_by_title(title: &str) -> Result<Window> {
    let windows = get_windows()?;
    for window in windows {
        if window.title() == title {
            return Ok(window);
        }
    }
    Err(anyhow!("Window not found"))
}

/// Screen capture the window.
pub fn screenshot_window(window: Window) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = window.capture_image()?;
    Ok(image)
}
