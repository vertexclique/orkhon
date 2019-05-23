#[derive(Debug, PartialEq, PartialOrd)]
pub enum ORequest
{
    ForPyModel(PyModelRequest),
    ForTFModel(TFRequest),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OResponse
{
    ForPyModel(PyModelResponse),
    ForTFModel(TFResponse),
}

pub(crate) trait ORequestBase<T> {}
pub(crate) trait OResponseBase<T> {}

impl<T> ORequestBase<T> for T {}
impl<T> OResponseBase<T> for T {}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct PyModelRequest {}

impl PyModelRequest {
    pub fn new() -> Self {
        PyModelRequest { ..Default::default() }
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct PyModelResponse {}

impl PyModelResponse {
    pub fn new() -> Self { PyModelResponse { ..Default::default() } }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct TFRequest {}

impl TFRequest {
    pub fn new() ->  Self { TFRequest { .. Default::default()} }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct TFResponse {}

impl TFResponse {
    pub fn new() ->  Self { TFResponse  { .. Default::default()} }
}
