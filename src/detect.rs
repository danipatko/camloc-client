use opencv::types::VectorOfVectorOfPoint;
use opencv::{
    core::{Point_, Rect},
    imgproc,
    prelude::*,
};

static BIN_TRESHOLD: f64 = 0.8 * 255.0;
static MIN_AREA: i32 = 20;
static MAX_AREA: i32 = 1000;
static MAX_RATIO: f64 = 0.05;
static ADJACENT_DIST: i32 = 200;
static ADJACENT_AREA_DIFF: i32 = 200;

/// detects checkerboard squares  
/// draws squares to `dst`  
/// returns averaged point
pub fn detect_checkerboard(frame: &Mat) -> opencv::Result<Option<Rect>> {
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

    let mut boxes = Vec::<Rect>::new();
    for c in contours {
        if let Ok(rect) = imgproc::bounding_rect(&c) {
            let ratio = rect.width as f64 / rect.height as f64;
            if rect.area() > MIN_AREA
                && rect.area() < MAX_AREA
                && ratio < 1.0 + MAX_RATIO
                && ratio > 1.0 - MAX_RATIO
            {
                boxes.push(rect);
            }
        }
    }

    Ok(find_adjacent(&mut boxes))
}

/// returns the bounding rect around points that are close to each other
fn find_adjacent(original: &mut Vec<Rect>) -> Option<Rect> {
    if original.len() < 2 {
        return None;
    }

    let mut res = Vec::<Rect>::new();
    res.push(original.remove(0));

    for item in original {
        if is_close(&res, item) {
            res.push(item.clone());
        }
    }

    // find min and max augh
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (res[0].x, 0, res[0].y, 0);
    for item in res {
        if item.x < min_x {
            min_x = item.x;
        }
        if item.x > max_x {
            max_x = item.x;
        }
        if item.y < min_y {
            min_y = item.y;
        }
        if item.y > max_y {
            max_y = item.y;
        }
    }

    Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
}

/// determined by distance and area difference
fn is_close(res: &Vec<Rect>, current: &Rect) -> bool {
    for o in res {
        if (o.x - current.x).abs() + (o.y - current.y).abs() > ADJACENT_DIST
            || (o.area() - current.area()).abs() > ADJACENT_AREA_DIFF
        {
            return false;
        }
    }
    true
}
