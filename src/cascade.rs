use opencv::{
    core::{Rect, Size},
    imgproc,
    objdetect::{self, CascadeClassifier},
    prelude::*,
    types,
};

pub fn detect_faces(frame: &Mat, cascade: &mut CascadeClassifier) -> opencv::Result<Option<Rect>> {
    let mut gray = Mat::default();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut reduced = Mat::default();
    imgproc::resize(
        &gray,
        &mut reduced,
        Size {
            height: 0,
            width: 0,
        },
        0.25,
        0.25,
        imgproc::INTER_LINEAR,
    )?;

    let mut faces = types::VectorOfRect::new();
    cascade.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        2,
        objdetect::CASCADE_SCALE_IMAGE,
        Size {
            width: 30,
            height: 30,
        },
        Size {
            width: 0,
            height: 0,
        },
    )?;

    println!("len: {}", faces.len());
    if faces.len() > 0 {
        let face = faces.to_vec()[0];
        println!("{:?}", face);
        Ok(Some(face))
    } else {
        Ok(None)
    }
}
