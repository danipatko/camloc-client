// #![allow(unused)]
mod detect;
mod track;

// use cv_convert::{prelude::*, FromCv, IntoCv};
// use ndarray_npy;
// use opencv::core::ToInputOutputArray;
use opencv::{core::Vec2s, highgui, imgproc, prelude::*, videoio};

// use std::io::Write;
// use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map1 = load_first("/home/dapa/code/camloc-client/picam1.sex")?;
    let map2 = load_second("/home/dapa/code/camloc-client/picam2.sex")?;

    let mut dstmap1 = Mat::default();
    let mut dstmap2 = Mat::default();
    opencv::imgproc::convert_maps(
        &map1,
        &map2,
        &mut dstmap1,
        &mut dstmap2,
        opencv::core::CV_32FC1,
        false,
    )?;

    // base tcp server
    /*
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1111);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    // */

    // loop {
    // let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
    // println!("Connection received from {:?}", addr);

    highgui::named_window("videocap", highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    // let rect = match detect::detect_checkerboard(&frame)? {
    //     Some(bx) => bx,
    //     None => opencv::core::Rect::default(),
    // };

    // let mut tracker = track::RolandTrack::create(&frame, rect);
    let mut undistorted = Mat::default();

    loop {
        cam.read(&mut frame)?;
        imgproc::remap(
            &frame,
            &mut undistorted,
            &dstmap1,
            &dstmap2,
            opencv::imgproc::INTER_LINEAR,
            opencv::core::BORDER_CONSTANT,
            opencv::core::Scalar::default(),
        )?;

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

/// map1: has 3 dimensions, dtype is i16
fn load_first(file: &str) -> Result<Mat, Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut int_buf: [u8; 4] = [0, 0, 0, 0];
    let mut short_buf: [u8; 2] = [0, 0];
    let mut f = std::fs::File::open(file)?;

    f.read_exact(&mut int_buf)?;
    let w = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let h = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let d = i32::from_le_bytes(int_buf);
    println!("mat1: w = {}, h = {}, d = {}", w, h, d);

    // ((map1.type() == CV_32FC2 || map1.type() == CV_16SC2) && map2.empty()) || (map1.type() == CV_32FC1 && map2.type() == CV_32FC1)

    let mut m = Mat::new_nd_vec_with_default(
        &opencv::core::Vector::from_slice(&[w, h]),
        opencv::core::CV_16SC2,
        opencv::core::VecN::default(),
    )?;

    println!(
        "channels: {} | depth: {} | size: {:?}",
        m.channels(),
        m.depth(),
        m.mat_size()
    );

    // read 3d array
    for x in 0..w {
        for y in 0..h {
            for dep in 0..d {
                f.read_exact(&mut short_buf)?;
                (*m.at_2d_mut::<Vec2s>(x, y)?)[dep as usize] = i16::from_le_bytes(short_buf);
            }
        }
    }

    let mut test = vec![];
    f.read_to_end(&mut test)?;
    println!("left {} bytes", test.len());

    Ok(m)
}

/// map2 has 2 dims, dtype is u16
fn load_second(file: &str) -> Result<Mat, Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut int_buf: [u8; 4] = [0, 0, 0, 0];
    let mut short_buf: [u8; 2] = [0, 0];
    let mut f = std::fs::File::open(file)?;

    f.read_exact(&mut int_buf)?;
    let w = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let h = i32::from_le_bytes(int_buf);
    println!("w = {}, h = {}", w, h);

    let mut m = Mat::new_nd_vec_with_default(
        &opencv::core::Vector::from_slice(&[w, h]),
        opencv::core::CV_16UC1,
        opencv::core::VecN::default(),
    )?;

    // read 2d array
    for x in 0..w {
        for y in 0..h {
            f.read_exact(&mut short_buf)?;
            let p = u16::from_le_bytes(short_buf);
            *m.at_2d_mut::<u16>(x, y)? = p as u16;
        }
    }

    let mut test = vec![];
    f.read_to_end(&mut test)?;
    println!("2: left {} bytes", test.len());

    Ok(m)
}

// fn _load(file: &str) -> Result<Mat, Box<dyn std::error::Error>> {
//     use std::io::Read;
//     let mut int_buf: [u8; 4] = [0, 0, 0, 0];
//     let mut short_buf: [u8; 2] = [0, 0];
//     let mut f = std::fs::File::open(file)?;
//
//     f.read_exact(&mut int_buf)?;
//     let w = i32::from_le_bytes(int_buf);
//     f.read_exact(&mut int_buf)?;
//     let h = i32::from_le_bytes(int_buf);
//     f.read_exact(&mut int_buf)?;
//     let d = i32::from_le_bytes(int_buf);
//     println!("w = {}, h = {}, d = {}", w, h, d);
//
//     let mut dims = opencv::core::Vector::new();
//     dims.push(w);
//     dims.push(h);
//     if d > 1 {
//         dims.push(d);
//     }
//
//     let mut m = Mat::new_nd_vec_with_default(
//         &dims,
//         match t {
//             opencv::core::CV_16SC1 => opencv::core::CV_16SC1,
//             _ => opencv::core::CV_16U,
//         },
//         opencv::core::VecN::default(),
//     )?;
//
//     // read 3d array
//     if d > 1 {
//         for x in 0..w {
//             for y in 0..h {
//                 for dep in 0..d {
//                     f.read_exact(&mut short_buf)?;
//                     let p = i16::from_le_bytes(short_buf);
//                     *m.at_3d_mut::<i16>(x, y, dep)? = p;
//                 }
//             }
//         }
//     // read 2d array
//     } else {
//         for x in 0..w {
//             for y in 0..h {
//                 let p = i16::from_le_bytes(short_buf);
//                 *m.at_2d_mut::<i16>(x, y)? = p;
//             }
//         }
//     }
//
//     Ok(m)
// }
