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
use tract_core::internal::PhantomData;
use orkhon::errors::*;

#[runtime::test(runtime_tokio::Tokio)]
async fn async_request_for_tf_model() {
    let _ = env_logger::builder().is_test(true).try_init();

    let o = Orkhon::new()
        .config(OrkhonConfig::new())
        .tensorflow("mobilenet",
                    PathBuf::from("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb")
        )
        .build();

    let handle =
        o.request_async("mobilenet",
                        ORequest::ForTFModel(TFRequest::new()));

    handle.await.unwrap();
}
