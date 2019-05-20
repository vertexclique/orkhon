use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

pub(crate) trait OrkhonAPI {
    fn request(&mut self, model_name: &str, request: ORequest) -> Result<OResponse>;

    fn build(mut self) -> Self;
}
