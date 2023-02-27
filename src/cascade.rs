use opencv::{
    core::{Rect, Size},
    imgproc,
    objdetect::CascadeClassifier,
    prelude::*,
    types,
};

pub fn detect_faces(frame: &Mat, cascade: &mut CascadeClassifier) -> opencv::Result<Option<Rect>> {
    let mut gray = Mat::default();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut faces = types::VectorOfRect::new();
    cascade.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        2,
        opencv::objdetect::CASCADE_SCALE_IMAGE,
        Size {
            width: 30,
            height: 30,
        },
        Size {
            width: 0,
            height: 0,
        },
    )?;

    if faces.len() > 0 {
        Ok(Some(faces.to_vec()[0]))
    } else {
        Ok(None)
    }
}
