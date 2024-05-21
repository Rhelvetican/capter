use std::{
    error::Error,
    io::{stdout, Write},
    time::Instant,
};

use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{VideoEncoder, VideoEncoderQuality, VideoEncoderType},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
};

pub struct WindowsCapture {
    pub encoder: Option<VideoEncoder>,
    pub start: Instant,
}

impl GraphicsCaptureApiHandler for WindowsCapture {
    type Flags = String;
    type Error = Box<dyn Error + Send + Sync>;

    fn new(flags: Self::Flags) -> Result<Self, Self::Error> {
        println!("Windows capture started with flags: {}", flags);
        let encoder = VideoEncoder::new(
            VideoEncoderType::Mp4,
            VideoEncoderQuality::HD1080p,
            1920,
            1080,
            "output.mp4",
        )?;

        Ok(Self {
            encoder: Some(encoder),
            start: Instant::now(),
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        stdout().flush()?;
        self.encoder.as_mut().unwrap().send_frame(frame)?;

        if self.start.elapsed().as_secs() >= 10 {
            self.encoder.take().unwrap().finish()?;
            capture_control.stop();
            println!();
        }

        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}
