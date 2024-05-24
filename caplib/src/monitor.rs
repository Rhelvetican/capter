//! This module provides functions to get information about monitors and to take screenshots of them.

use anyhow::Result;
pub use xcap::{
    image::{ImageBuffer, Rgba},
    Monitor,
};

/// Get all monitors on the system.
pub fn get_monitors() -> Result<Vec<Monitor>> {
    match Monitor::all() {
        Ok(monitors) => Ok(monitors),
        Err(e) => Err(e.into()),
    }
}

/// Screen capture the monitor.
pub fn screenshoot_monitor(monitor: Monitor) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = monitor.capture_image()?;
    Ok(image)
}
