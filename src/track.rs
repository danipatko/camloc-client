use crate::detect;
use opencv::core::VecN;
use opencv::video::Tracker;
use opencv::{
    core::{Point_, Ptr, Rect, ToInputOutputArray},
    imgproc,
    prelude::*,
    tracking,
};

pub struct RolandTrack {
    has_object: bool,
    tracker: Ptr<dyn tracking::TrackerKCF>,
    bounding_box: Rect,
}

trait Center {
    fn center(&self) -> Point_<i32>;
    fn find_x(&self, frame: &Mat) -> f64;
}

impl Center for Rect {
    fn center(&self) -> Point_<i32> {
        return Point_ {
            x: self.x + (self.width / 2),
            y: self.y + (self.height / 2),
        };
    }

    /// x position relative to screen width
    /// 0 < x < 1
    fn find_x(&self, frame: &Mat) -> f64 {
        let x = self.center().x;
        x as f64 / frame.size().unwrap().width as f64
    }
}

impl RolandTrack {
    pub fn create() -> Self {
        let params = tracking::TrackerKCF_Params::default().unwrap();

        match <dyn tracking::TrackerKCF>::create(params) {
            Ok(t) => Self {
                has_object: true,
                tracker: t,
                bounding_box: Rect::default(),
            },
            Err(e) => panic!(
                "Failed to initialie TrackerKCF in RolandTrack::create()\n{}",
                e
            ),
        }
    }

    pub fn update(
        &mut self,
        frame: &Mat,
        dst: &mut dyn ToInputOutputArray,
    ) -> Result<Option<f64>, Box<dyn std::error::Error>> {
        println!("lost object: {}", self.has_object);
        // use trackerKCF to track known position
        if self.has_object {
            match self.tracker.update(frame, &mut self.bounding_box) {
                // no errors and found image
                Ok(true) => {
                    draw(dst, self.bounding_box);
                    Ok(Some(self.bounding_box.find_x(frame)))
                }
                // error or lost object
                _ => {
                    self.has_object = false;
                    Ok(None)
                }
            }
        } else {
            // use detect to find again
            match detect::detect_checkerboard(&frame)? {
                Some(bx) => {
                    println!("detected rectangle again yay");
                    self.has_object = true;
                    draw(dst, bx);
                    self.tracker.init(frame, bx);

                    self.bounding_box = bx.clone();
                    Ok(Some(bx.find_x(frame)))
                }
                None => {
                    println!("unable to detect image");
                    Ok(None)
                }
            }
        }
    }
}

fn draw(dst: &mut dyn ToInputOutputArray, rect: Rect) -> Result<(), Box<dyn std::error::Error>> {
    imgproc::rectangle(
        dst,
        rect,
        VecN::new(255.0, 255.0, 0.0, 1.0),
        2,
        imgproc::LINE_8,
        0,
    )?;
    Ok(())
}
