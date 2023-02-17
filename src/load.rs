use opencv::{core::Vec2s, prelude::*};
use std::io::Read;

/// map1 is short type, has two channels
fn load_first(file: &str) -> Result<Mat, Box<dyn std::error::Error>> {
    let mut int_buf: [u8; 4] = [0, 0, 0, 0];
    let mut short_buf: [u8; 2] = [0, 0];
    let mut f = std::fs::File::open(file)?;

    f.read_exact(&mut int_buf)?;
    let w = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let h = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let d = i32::from_le_bytes(int_buf);

    let mut m = Mat::new_nd_vec_with_default(
        &opencv::core::Vector::from_slice(&[w, h]),
        opencv::core::CV_16SC2,
        opencv::core::VecN::default(),
    )?;

    for x in 0..w {
        for y in 0..h {
            for dep in 0..d {
                f.read_exact(&mut short_buf)?;
                (*m.at_2d_mut::<Vec2s>(x, y)?)[dep as usize] = i16::from_le_bytes(short_buf);
            }
        }
    }

    Ok(m)
}

/// map2 is unsigned short, 1 channel
fn load_second(file: &str) -> Result<Mat, Box<dyn std::error::Error>> {
    let mut int_buf: [u8; 4] = [0, 0, 0, 0];
    let mut short_buf: [u8; 2] = [0, 0];
    let mut f = std::fs::File::open(file)?;

    f.read_exact(&mut int_buf)?;
    let w = i32::from_le_bytes(int_buf);
    f.read_exact(&mut int_buf)?;
    let h = i32::from_le_bytes(int_buf);

    let mut m = Mat::new_nd_vec_with_default(
        &opencv::core::Vector::from_slice(&[w, h]),
        opencv::core::CV_16UC1,
        opencv::core::VecN::default(),
    )?;

    for x in 0..w {
        for y in 0..h {
            f.read_exact(&mut short_buf)?;
            let p = u16::from_le_bytes(short_buf);
            *m.at_2d_mut::<u16>(x, y)? = p as u16;
        }
    }

    Ok(m)
}

/// reads binary files and converts them from (CV_16SC2, CV_16UC1) to (CV_32FC1, CV_32FC1)
/// pass this to remap
pub fn maps(path: &str) -> Result<(Mat, Mat), Box<dyn std::error::Error>> {
    let map1 = load_first(format!("{}{}", path, "1.sex").as_str())?;
    let map2 = load_second(format!("{}{}", path, "2.sex").as_str())?;

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

    Ok((dstmap1, dstmap2))
}
