use anyhow::Result;
use xcap::image::{ImageBuffer, Rgba};
pub use xcap::Monitor;

pub fn get_monitors() -> Result<Vec<Monitor>> {
    match Monitor::all() {
        Ok(monitors) => Ok(monitors),
        Err(e) => Err(e.into()),
    }
}

pub fn screenshoot(monitor: Monitor) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = monitor.capture_image()?;
    Ok(image)
}
