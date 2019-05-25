#![feature(async_await)]

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
use orkhon::errors::*;

#[runtime::test(runtime_tokio::Tokio)]
async fn async_request_for_tf_model() {
    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(OrkhonConfig::new())
        .pymodel("model_which_will_be_tested",
                 "tests/pymodels",
                 "model_test",
            "model_hook"
        )
        .build();

    let mut request_args = HashMap::new();
    request_args.insert("is", 10);
    request_args.insert("are", 6);
    request_args.insert("you", 5);

    let mut request_kwargs = HashMap::<&str, &str>::new();

    let handle =
        o.pymodel_request_async(
            "model_which_will_be_tested",
            ORequest::with_body(
                PyModelRequest::new()
                    .with_args(request_args)
                    .with_kwargs(request_kwargs)
            )
        );

    handle.await.unwrap();
}
