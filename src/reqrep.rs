use tract_core::internal::HashMap;

pub enum Types {
    PyModel,
    TFModel
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ORequest<T>
{
    pub body: T,
    _private: ()
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct OResponse<T>
{
    pub body: T,
    _private: ()
}

pub(crate) trait ORequestBase<T> {}
pub(crate) trait OResponseBase<T> {}

impl<T> ORequestBase<T> for T {}
impl<T> OResponseBase<T> for T {}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct PyModelRequest<K, V> {
    args: HashMap<K, V>
}

impl<K, V> PyModelRequest<K, V> {
    pub fn new() -> Self {
        PyModelRequest { ..Default::default() }
    }

    pub fn with_args(mut self, args: HashMap<K, V>) -> Self {
        self.args = args;
        self
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
