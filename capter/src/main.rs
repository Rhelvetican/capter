use caplib::{get_monitors, screenshoot};

fn main() {
    let monitors = get_monitors().unwrap();
    for monitor in monitors {
        let image = screenshoot(monitor).unwrap();
        println!("Image size: {}x{}", image.width(), image.height());
        image.save("screenshot.png").unwrap();
    }
}
