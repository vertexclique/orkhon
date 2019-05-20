use crate::reqrep::{ORequest, OResponse};
use crate::errors::*;

pub(crate) trait Service {
    fn load(&mut self) -> Result<()>;
    fn process(&mut self, request: ORequest) -> Result<OResponse>;
}
