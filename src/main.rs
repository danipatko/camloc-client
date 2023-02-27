mod cascade;
mod detect;
mod load;
mod track;

use clap::Parser;
use opencv::{highgui, prelude::*, videoio};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Disable GUI, create a TCP server. On connection, tracking will start and sends the relative x positions to the connected client.
    /// Only one client can connect at a time.
    #[arg(short, long)]
    server: bool,

    /// Specify TCP port
    #[arg(long, default_value_t = 1111)]
    port: u16,

    /// Specify the TCP host
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    /// Specify the location of remap binary files
    #[arg(long, default_value_t = String::from("picam"))]
    path: String,

    /// Camera index to open
    #[arg(long, default_value_t = 0)]
    camera_index: i32,

    /// Detect faces instead of checkerboard
    #[arg(long)]
    faces: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let maps = load::maps(args.path.as_str())?;
    let cam = videoio::VideoCapture::new(args.camera_index, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    if args.server {
        tcp_loop(cam, maps, args.host, args.port, args.faces)?;
    } else {
        dev_loop(cam, maps, args.faces)?;
    }

    Ok(())
}

/// starts tcp server with no GUI
fn tcp_loop(
    mut cam: opencv::videoio::VideoCapture,
    (map1, map2): (Mat, Mat),
    host: String,
    port: u16,
    faces: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::net::TcpListener;
    println!("Starting in server mode...");

    // initialize tcp server
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    let port = listener.local_addr()?;

    // `frame` is remapped to `undistorted`
    let mut frame = Mat::default();
    let mut undistorted = Mat::default();

    // initialize tracker
    cam.read(&mut frame)?;
    let mut tracker = track::RolandTrack::create(&frame, opencv::core::Rect::default(), faces);

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

            let x = tracker.update(&undistorted, None)?;
            if tcp_stream
                .write_all(&x.unwrap_or(f64::NAN).to_be_bytes())
                .is_err()
            {
                println!("Client disconnected");
                break;
            }

            // still need this delay
            let _key = highgui::wait_key(10)?;
        }
    }

    // Ok(())
}

fn dev_loop(
    mut cam: opencv::videoio::VideoCapture,
    (map1, map2): (Mat, Mat),
    faces: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting in development mode...");

    let winname = "video";
    highgui::named_window(winname, highgui::WINDOW_AUTOSIZE)?;

    // `frame` is remapped to `undistorted`
    let mut frame = Mat::default();
    let mut undistorted = Mat::default();

    // initialize tracker
    cam.read(&mut frame)?;
    let mut tracker = track::RolandTrack::create(&frame, opencv::core::Rect::default(), faces);

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
        if let Some(x) = tracker.update(&undistorted, Some(&mut draw))? {
            println!("x = {:.10}", x)
        }

        let _key = highgui::wait_key(10)?;
        if draw.size()?.width > 0 {
            highgui::imshow(winname, &draw)?;
        }
    }

    // Ok(())
}
