// #[runtime::test(runtime_tokio::Tokio)]
// fn test_async_request_for_tf_loaded_py_model() {
//     let _ = env_logger::builder().is_test(true).try_init();
//
//     let o = Orkhon::new()
//         .config(OrkhonConfig::new())
//         .pymodel("tf_loaded_py_model",
//                  "tests/pymodels",
//                  "tf_model",
//             "tf_model_hook"
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
//             "tf_loaded_py_model",
//             ORequest::with_body(
//                 PyModelRequest::new()
//                     .with_args(request_args)
//                     .with_kwargs(request_kwargs)
//             )
//         );
//
//     handle.await.unwrap();
// }
