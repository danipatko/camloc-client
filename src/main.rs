mod detect;
mod track;

use opencv::{highgui, prelude::*, videoio};

// use std::io::{Error, Read, Write};
// use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

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

    let mut tracker = track::RolandTrack::create();

    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    loop {
        cam.read(&mut frame)?;
        let mut draw = frame.clone();

        // found object
        match tracker.update(&mut frame, &mut draw)? {
            Some(x) => {
                println!("x = {}", x)
            }
            None => (),
        }

        if draw.size()?.width > 0 {
            highgui::imshow("videocap", &draw)?;
        }

        // let key = highgui::wait_key(10)?;
        // if key > 0 && key != 255 {
        //     break;
        // }
    }
}
