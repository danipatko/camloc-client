#![allow(unused)]

mod detect;
mod track;

use opencv::core::VecN;
use opencv::video::Tracker;
use opencv::{core, highgui, imgproc, prelude::*, tracking, videoio};

use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // base tcp server
    /*
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1111);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);

    loop {
        let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
        println!("Connection received from {:?}", addr);

        loop {
            match tcp_stream.write("augh ".as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Connection terminated: {}", e.kind());
                    break;
                }
            }
        }
    }
    // */

    highgui::named_window("videocap", highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    // let mut tracker = track::RolandTrack::create();

    // let params = tracking::TrackerKCF_Params::default().unwrap();
    // let mut t = <dyn tracking::TrackerKCF>::create(params)?;

    // let mut bounding_box = highgui::select_roi(&mut frame, false, false).unwrap();
    // t.init(&mut frame, bounding_box);

    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    loop {
        cam.read(&mut frame)?;

        // detection
        let mut draw = frame.clone();
        match detect::detect_checkerboard(&frame, &mut draw)? {
            Some(bx) => imgproc::rectangle(
                &mut draw,
                bx,
                VecN::new(255.0, 255.0, 0.0, 1.0),
                2,
                imgproc::LINE_8,
                0,
            )?,
            None => (),
        }

        // found object
        // match t.update(&mut frame, &mut bounding_box) {
        //     Ok(true) => {
        //         imgproc::rectangle(
        //             &mut frame,
        //             bounding_box,
        //             core::Scalar::new(0f64, -1f64, -1f64, -1f64),
        //             2,
        //             8,
        //             0,
        //         );
        //     }
        //     Ok(false) => println!("object is out of the frame"),
        //     Err(e) => println!("{}", e),
        // }

        if frame.size()?.width > 0 {
            highgui::imshow("videocap", &draw)?;
        }

        let key = highgui::wait_key(10)?;
        // if key > 0 && key != 255 {
        //     break;
        // }
    }

    println!("END");
    Ok(())
}
