use opencv::core::VecN;
use opencv::types::VectorOfVectorOfPoint;
use opencv::video::Tracker;
use opencv::{
    core::{Point_, ToInputOutputArray, Vector},
    highgui, imgproc,
    prelude::*,
};

static BIN_TRESHOLD: f64 = 0.8 * 255.0;
static MIN_AREA: i32 = 20;
static MAX_AREA: i32 = 1000;
static MAX_RATIO: f64 = 0.1;

/// detects checkerboard squares  
/// draws squares to `dst`  
/// returns averaged point
pub fn detect_checkerboard(
    frame: &Mat,
    dst: &mut dyn ToInputOutputArray,
) -> Result<Point_<i32>, Box<dyn std::error::Error>> {
    // create gray image
    let mut gray = frame.clone();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut binary = Mat::default();
    imgproc::threshold(
        &gray,
        &mut binary,
        BIN_TRESHOLD,
        255f64,
        imgproc::THRESH_BINARY,
    )?;

    // find contours
    // VectorOfVectorOfPoint == std::vector<std::vectorcv::Point> WHY
    let mut contours = VectorOfVectorOfPoint::new();
    imgproc::find_contours(
        &binary,
        &mut contours,
        imgproc::RETR_LIST,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point_::<i32> { x: 0, y: 0 },
    )?;

    let mut rect = opencv::core::Rect::default();
    let (mut sum_x, mut sum_y, mut count) = (0, 0, 0);

    for c in contours {
        rect = imgproc::bounding_rect(&c)?;

        if rect.area() > MIN_AREA
            && rect.area() < MAX_AREA
            && (1.0 - (rect.width as f64 / rect.height as f64)).abs() < MAX_RATIO
        {
            sum_x += rect.x;
            sum_y += rect.y;
            count += 1;

            imgproc::rectangle(
                dst,
                rect,
                VecN::new(0.0, 255.0, 0.0, 1.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }
    }

    Ok(Point_::<i32> {
        x: sum_x / count,
        y: sum_y / count,
    })
}
