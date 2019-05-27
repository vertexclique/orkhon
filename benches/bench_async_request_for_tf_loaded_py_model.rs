#![feature(async_await, test)]

extern crate test;

#[macro_use]
extern crate static_assertions;

use orkhon::orkhon::Orkhon;
use orkhon::config::OrkhonConfig;
use orkhon::tensorflow::TFModel;
use std::path::PathBuf;
use std::{env, fs};
use log::*;
use orkhon::pooled::PooledModel;
use orkhon::reqrep::{ORequest, TFRequest, PyModelRequest, OResponse, TFResponse};
use tract_core::internal::{PhantomData, HashMap};
use futures::future::join_all;

#[runtime::bench(runtime_tokio::Tokio)]
async fn test_async_request_for_tf_loaded_py_model() {
    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(OrkhonConfig::new())
        .pymodel("tf_loaded_py_model",
                 "tests/pymodels",
                 "tf_model",
            "tf_model_hook"
        )
        .build();

    let mut request_args = HashMap::new();
    request_args.insert("is", 10);
    request_args.insert("are", 6);
    request_args.insert("you", 5);

    let request_kwargs = HashMap::<&str, &str>::new();

    let req = PyModelRequest::new()
        .with_args(request_args)
        .with_kwargs(request_kwargs);

    let handle =
        o.pymodel_request_async(
            "tf_loaded_py_model",
            ORequest::with_body(req)
        );

    handle.await.unwrap();
}
