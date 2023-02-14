use crate::detect;
use opencv::video::Tracker;
use opencv::{
    core::{Ptr, ToInputOutputArray},
    highgui, imgproc,
    prelude::*,
    tracking, videoio,
};

pub struct RolandTrack {
    lost_object: bool,
    tracker: Ptr<dyn tracking::TrackerKCF>,
}

impl RolandTrack {
    pub fn create() -> Self {
        let params = tracking::TrackerKCF_Params::default().unwrap();

        match <dyn tracking::TrackerKCF>::create(params) {
            Ok(t) => Self {
                lost_object: true,
                tracker: t,
            },
            Err(e) => panic!(
                "Failed to initialie TrackerKCF in RolandTrack::create()\n{}",
                e
            ),
        }
    }

    fn update(
        &self,
        frame: &Mat,
        dst: &mut dyn ToInputOutputArray,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        // use detect to find again
        if self.lost_object {
            detect::detect_checkerboard(frame, dst)?;
        }

        Ok(0f64)
    }
}
