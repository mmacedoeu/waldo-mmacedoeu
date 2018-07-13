use actix::{Actor, Handler, Message, Supervised, SyncContext};
use cv::{Mat, CvType, TermCriteria, TermType};
use errors::{Error, ErrorKind, Result};
use serde_json::value::Value;

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

impl SearchExecutor {
    fn align(&mut self, req: &Images) {
        // TODO
    }    
}
