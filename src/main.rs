mod detect;
mod load;
mod track;

use opencv::{highgui, imgproc, prelude::*, videoio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (map1, map2) = load::maps("/home/dapa/code/camloc-client/picam")?;

    highgui::named_window("videocap", highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    let mut tracker = track::RolandTrack::create(&frame, opencv::core::Rect::default());
    let mut undistorted = Mat::default();

    // tcp_connect_loop(|| {
    //     cam.read(&mut frame);
    //     imgproc::remap(
    //         &frame,
    //         &mut undistorted,
    //         &map1,
    //         &map2,
    //         opencv::imgproc::INTER_LINEAR,
    //         opencv::core::BORDER_CONSTANT,
    //         opencv::core::Scalar::default(),
    //     );

    //     let x = tracker.update(&frame);

    //     Some(0.0)
    // })?;

    loop {
        // let mut draw = frame.clone();

        // found object
        // let Some(_) = tracker.update(&mut frame/*, &mut draw */)? else {
        //     continue;
        // };

        // if tcp_stream.write_all(&x.to_be_bytes()).is_err() {
        //     break;
        // }

        // IMPORTANT: the whole shit breaks for whatever reason without this delay
        let _key = highgui::wait_key(10)?;

        if undistorted.size()?.width > 0 {
            highgui::imshow("videocap", &undistorted)?;
        }
        // }
    }
    // */
    // Ok(())
}

fn tcp_connect_loop(f: fn() -> Option<f64>) -> std::io::Result<()> {
    use std::io::Write;
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1111);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);

    loop {
        let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
        println!("Connection received from {:?}", addr);

        loop {
            if let Some(x) = f() {
                if tcp_stream.write_all(&x.to_be_bytes()).is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}
