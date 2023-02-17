mod detect;
mod load;
mod track;

use opencv::{prelude::*, videoio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maps = load::maps("/home/dapa/code/camloc-client/picam")?;

    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    #[cfg(feature = "pi")]
    tcp_loop(cam, maps)?;

    #[cfg(not(feature = "pi"))]
    dev_loop(cam, maps)?;

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

    // loop {
    // let mut draw = frame.clone();

    // found object
    // let Some(_) = tracker.update(&mut frame/*, &mut draw */)? else {
    //     continue;
    // };

    // if tcp_stream.write_all(&x.to_be_bytes()).is_err() {
    //     break;
    // }

    // // IMPORTANT: the whole shit breaks for whatever reason without this delay
    // let _key = highgui::wait_key(10)?;

    // if undistorted.size()?.width > 0 {
    //     highgui::imshow("videocap", &undistorted)?;
    // }
    // }
    // }
    // */
    Ok(())
}

/// starts tcp server with no GUI
#[cfg(feature = "pi")]
fn tcp_loop(
    mut cam: opencv::videoio::VideoCapture,
    (map1, map2): (Mat, Mat),
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    println!("Starting in raspberry mode...");

    // initialize tcp server
    let listener = TcpListener::bind("0.0.0.0:1111")?;
    let port = listener.local_addr()?;

    // `frame` is remapped to `undistorted`
    let mut frame = Mat::default();
    let mut undistorted = Mat::default();

    // initialize tracker
    cam.read(&mut frame)?;
    let mut tracker = track::RolandTrack::create(&frame, opencv::core::Rect::default());

    loop {
        // block until requested
        println!("Waiting for connections on {}", port);
        let (mut tcp_stream, addr) = listener.accept()?;
        println!("Connection received from {:?}", addr);

        // spam results
        loop {
            cam.read(&mut frame)?;
            opencv::imgproc::remap(
                &frame,
                &mut undistorted,
                &map1,
                &map2,
                opencv::imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                opencv::core::Scalar::default(),
            )?;

            if let Some(x) = tracker.update(&undistorted)? {
                if tcp_stream.write_all(&x.to_be_bytes()).is_err() {
                    println!("Client disconnected");
                    break;
                }

            // TOFIX: find some better way to check if the client is still connected
            // without checking this, the loop won't break until the tracker detects
            // something, even if the client has disconnected
            } else if tcp_stream.read(&mut vec![]).is_err() {
                break;
            }
        }
    }

    // Ok(())
}

#[cfg(not(feature = "pi"))]
fn dev_loop(
    mut cam: opencv::videoio::VideoCapture,
    (map1, map2): (Mat, Mat),
) -> Result<(), Box<dyn std::error::Error>> {
    use opencv::highgui;
    println!("Starting in development mode...");

    let winname = "video";
    highgui::named_window(winname, highgui::WINDOW_AUTOSIZE)?;

    // `frame` is remapped to `undistorted`
    let mut frame = Mat::default();
    let mut undistorted = Mat::default();

    // initialize tracker
    cam.read(&mut frame)?;
    let mut tracker = track::RolandTrack::create(&frame, opencv::core::Rect::default());

    loop {
        cam.read(&mut frame)?;

        opencv::imgproc::remap(
            &frame,
            &mut undistorted,
            &map1,
            &map2,
            opencv::imgproc::INTER_LINEAR,
            opencv::core::BORDER_CONSTANT,
            opencv::core::Scalar::default(),
        )?;

        let mut draw = undistorted.clone();
        if let Some(x) = tracker.update(&undistorted, &mut draw)? {
            println!("x = {:.10}", x)
        }

        let _key = highgui::wait_key(10)?;
        if draw.size()?.width > 0 {
            highgui::imshow(winname, &draw)?;
        }
    }

    // Ok(())
}
