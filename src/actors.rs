use actix::{Actor, Handler, Message, Supervised, SyncContext};
use cv::{
    features2d::{DMatch, DescriptorMatcher, DescriptorMatcherType, Feature2D, SURFBuilder, SURF},
    Mat,
};
use errors::{Error, ErrorKind, Result};
use serde_json::value::Value;

const HESSIAN_THRESHOLD: f64 = 256.0;

pub struct Images {
    pub img1: Mat,
    pub img2: Mat,
}

impl Message for Images {
    type Result = Result<Value>;
}

pub struct SearchExecutor;

impl Actor for SearchExecutor {
    type Context = SyncContext<Self>;
}

impl Supervised for SearchExecutor {}

impl Handler<Images> for SearchExecutor {
    type Result = Result<Value>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, req: Images, _: &mut Self::Context) -> Self::Result {
        Ok(json!({"results": 0,
           }))
    }
}

/**
 * Implementation of responsibilities representing processing
 * to be done on graph representation dataflow
 */
impl SearchExecutor {
    /**
     * Main feature detection and description extration responsibility
     *
     */
    #[cfg_attr(feature = "flame_it", flame)]
    fn detect_matches(req: &Images) {
        let detector: SURF = SURFBuilder::default()
            .hessian_threshold(HESSIAN_THRESHOLD)
            .into();
        let (keypoints1, descriptors1) = detector.detect_and_compute(&req.img1, &Mat::new());
        let (keypoints2, descriptors2) = detector.detect_and_compute(&req.img2, &Mat::new());
        let matcher = DescriptorMatcher::new(DescriptorMatcherType::FlannBased);
        // TODO port knn_match binding with train descriptors parameter
        let knn_matches = matcher.match_two(&descriptors1, &descriptors2);
        let ratio_thresh = 0.8;
        let good_matches: Vec<Vec<DMatch>> = knn_matches
            .into_iter()
            .filter(|i| i.0.distance < 0.8 * i.1.distance)
            .collect();
    }

    /**
     * Pre processing of images to grayscale
     * and pyramid scaling
     */
    fn pre_processing(req: &Images) {
        //TODO
    }

    /**
     * Pyramid partition policy where image is divided
     * into pieces distributed on a cluster
     */
    fn partition(req: &Images) {
        //TODO
    }

    /**
     * Entry level function to determine presence
     * of a cropped image by calculating it's
     * translation transform on matched feactures descriptors
     */
    #[cfg_attr(feature = "flame_it", flame)]
    fn align(&mut self, req: &Images) {
        // TODO
    }
}
