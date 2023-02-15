mod detect;
mod track;

use opencv::{prelude::*, videoio};

// use crate::track::Center;

use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // base tcp server
    //*
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1111);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);

    loop {
        let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
        println!("Connection received from {:?}", addr);

        // highgui::named_window("videocap", highgui::WINDOW_AUTOSIZE)?;
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
            // let mut draw = frame.clone();

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

            let Some(x) = tracker.update(&mut frame/*, &mut draw */)? else {
                continue;
            };

            if tcp_stream.write_all(&x.to_be_bytes()).is_err() {
                break;
            }

            // IMPORTANT: the whole shit breaks for whatever reason without this delay
            // let _key = highgui::wait_key(10)?;

            // if draw.size()?.width > 0 {
            //     highgui::imshow("videocap", &draw)?;
            // }
        }
    }
    // */
    // Ok(())
}
