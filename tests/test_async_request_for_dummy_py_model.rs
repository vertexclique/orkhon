// #![feature(async_await)]
//
// use orkhon::orkhon::Orkhon;
// use orkhon::config::OrkhonConfig;
// use orkhon::reqrep::{ORequest, PyModelRequest};
// use std::collections::HashMap;
//
// #[runtime::test(runtime_tokio::Tokio)]
// async fn test_async_request_for_dummy_py_model() {
//     let _ = env_logger::builder().is_test(true).try_init();
//
//     let o = Orkhon::new()
//         .config(OrkhonConfig::new())
//         .pymodel("model_which_will_be_tested",
//                  "tests/pymodels",
//                  "model_test",
//             "model_hook"
//         )
//         .build();
//
//     let mut request_args = HashMap::new();
//     request_args.insert("is", 10);
//     request_args.insert("are", 6);
//     request_args.insert("you", 5);
//
//     let request_kwargs = HashMap::<&str, &str>::new();
//
//     let handle =
//         o.pymodel_request_async(
//             "model_which_will_be_tested",
//             ORequest::with_body(
//                 PyModelRequest::new()
//                     .with_args(request_args)
//                     .with_kwargs(request_kwargs)
//             )
//         );
//
//     handle.await.unwrap();
// }
