use crate::reqrep::{ORequest, OResponse};

pub(crate) trait Service {
    fn process(&mut self, request: ORequest) -> OResponse;
}
