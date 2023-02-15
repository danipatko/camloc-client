mod detect;
mod track;

use cv_convert::{prelude::*, FromCv, IntoCv};
use ndarray_npy;
use opencv::core::ToInputOutputArray;
use opencv::{imgproc, prelude::*, videoio};

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

    // let maps = load(&mut vec![], &mut vec![])?;
    let maps = load();

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
        let mut undistorted = Mat::default();

        loop {
            cam.read(&mut frame)?;
            imgproc::remap(
                &frame,
                &mut undistorted,
                opencv::core::InputArray::try_into_cv(maps.0).unwrap(),
                &maps.1.into_cv(),
                opencv::imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                opencv::core::Scalar::default(),
            );

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

use opencv::core::_InputOutputArray;
fn load() -> std::io::Result<(_InputOutputArray, _InputOutputArray)> {
    let w = 480;
    let h = 600;
    let z = 2;

    let dims = opencv::core::Vector::new();
    dims.push(w);
    dims.push(h);
    dims.push(z);
    let m1 =
        Mat::new_nd_vec_with_default(&dims, opencv::core::CV_16U, opencv::core::VecN::default());

    let m2 =
        Mat::new_nd_vec_with_default(&dims, opencv::core::CV_16U, opencv::core::VecN::default());

    Ok((m1.into(), m2.into()))
}

// fn load<'a>(
//     buf1: &'a mut Vec<u8>,
//     buf2: &'a mut Vec<u8>,
// ) -> std::io::Result<(NpyData<'a, f64>, NpyData<'a, f64>)> {
//     use std::io::Read;

//     let mut f = std::fs::File::open("map1.npy")?;
//     f.read_to_end(buf1)?;

//     let mut f = std::fs::File::open("map2.npy")?;
//     f.read_to_end(buf2)?;

//     // let map1 = ;
//     // let map2 = ;

//     Ok((NpyData::from_bytes(buf1)?, NpyData::from_bytes(buf2)?))
// }
