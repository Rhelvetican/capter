use caplib::monitor::{get_monitors, screenshoot_monitor};

fn main() {
    let monitors = get_monitors().unwrap();
    for monitor in monitors {
        let image = screenshoot_monitor(monitor).unwrap();
        println!("Image size: {}x{}", image.width(), image.height());
        image.save("screenshot.png").unwrap();
    }
}
