mod detect;
mod track;

use opencv::{highgui, prelude::*, videoio};

// use crate::track::Center;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    highgui::named_window("videocap", highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    let rect = match detect::detect_checkerboard(&frame)? {
        Some(bx) => bx,
        None => opencv::core::Rect::default(),
    };

    let mut tracker = track::RolandTrack::create(&frame, rect);

    loop {
        cam.read(&mut frame)?;
        let mut draw = frame.clone();

        // simple detection
        // match detect::detect_checkerboard(&frame)? {
        //     Some(bx) => {
        //         track::draw(&mut draw, bx)?;
        //         let x = bx.find_x(&frame);
        //         println!("detected {}", x);
        //     }
        //     None => (),
        // };

        // found object
        match tracker.update(&mut frame, &mut draw)? {
            Some(x) => println!("detected {}", x),
            _ => (),
        };

        // IMPORTANT: the whole shit breaks for whatever reason without this delay
        let _key = highgui::wait_key(10)?;

        if draw.size()?.width > 0 {
            highgui::imshow("videocap", &draw)?;
        }
    }

    // Ok(())
}
