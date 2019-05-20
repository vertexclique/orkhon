
#[derive(Default)]
pub struct ORequest {

}

impl ORequest {
    pub fn new() -> Self {
        ORequest { ..Default::default() }
    }
}

#[derive(Default)]
pub struct OResponse {}

impl OResponse {
    pub fn new() -> Self {
        OResponse { ..Default::default() }
    }
}
