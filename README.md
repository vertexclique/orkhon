<div align="center">
  <img src="https://github.com/vertexclique/orkhon/blob/master/doc/logo/orkhon.png"><br>
</div>

-----------------

<h1 align="center">Orkhon: ML Inference Framework and Server Runtime</h1>


<table align=left style='float: left; margin: 4px 10px 0px 0px; border: 1px solid #000000;'>
<tr>
  <td>Latest Release</td>
  <td>
    <a href="https://crates.io/crates/orkhon">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/orkhon.svg?style=popout-square">
    </a>
  </td>
</tr>
<tr>
  <td></td>
</tr>
<tr>
  <td>License</td>
  <td>
    <a href="https://github.com/vertexclique/orkhon/blob/master/LICENSE">
    <img alt="Crates.io" src="https://img.shields.io/crates/l/orkhon.svg?style=popout-square">
    </a>
</td>
</tr>
<tr>
  <td>Build Status</td>
  <td>
    <a href="https://github.com/vertexclique/orkhon/actions">
    <img alt="Build Status" src="https://github.com/vertexclique/orkhon/workflows/CI/badge.svg" />
    </a>
  </td>
</tr>
<tr>
  <td>Downloads</td>
  <td>
    <a href="https://crates.io/crates/orkhon">
    <img alt="Crates.io" src="https://img.shields.io/crates/d/orkhon.svg?style=popout-square">
    </a>
  </td>
</tr>
<tr>
	<td>Gitter</td>
	<td>
		<a href="https://gitter.im/orkhonml/community">
		<img src="https://badges.gitter.im/Join%20Chat.svg" />
		</a>
	</td>
</tr>
</table>

## What is it?

Orkhon is Rust framework for Machine Learning to run/use inference/prediction code written in Python, frozen models and process unseen data. It is mainly focused on serving models and processing unseen data in a performant manner. Instead of using Python directly and having scalability problems for servers this framework tries to solve them with built-in async API.

## Main features

* Sync & Async API for models.
* Easily embeddable engine for well-known Rust web frameworks.
* API contract for interacting with Python code.
* High processing throughput
    * ~4.8361 GiB/s prediction throughput
    * 3_000 concurrent requests takes ~4ms on average 
* Python Module caching

## Installation

You can include Orkhon into your project with;

```toml
[dependencies]
orkhon = "0.2"
```

## Dependencies
You will need:
* If you use `pymodel` feature, Python dev dependencies should be installed and have proper python runtime to use Orkhon with your project.
* If you want to have tensorflow inference. Installing tensorflow as library for linking is required.
* ONNX interface doesn't need extra dependencies from the system side.
* Point out your `PYTHONHOME` environment variable to your Python installation.
## Python API contract

For Python API contract you can take a look at the [Project Documentation](https://docs.rs/orkhon).

 ## Examples
 #### Request a Tensorflow prediction asynchronously

```rust
 use orkhon::prelude::*;
 use orkhon::tcore::prelude::*;
 use orkhon::ttensor::prelude::*;
 use rand::*;
 use std::path::PathBuf;

let o = Orkhon::new()
    .config(
        OrkhonConfig::new()
            .with_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
    )
    .tensorflow(
        "model_which_will_be_tested",
        PathBuf::from("tests/protobuf/manual_input_infer/my_model.pb"),
    )
    .shareable();

let mut rng = thread_rng();
let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

let o = o.get();
let handle = async move {
    let processor = o.tensorflow_request_async(
       "model_which_will_be_tested",
       ORequest::with_body(TFRequest::new().body(input.into())),
    );
    processor.await
};
let resp = block_on(handle).unwrap();
```

 #### Request an ONNX prediction synchronously

This example needs `onnxmodel` feature enabled.

```rust
use orkhon::prelude::*;
use orkhon::tcore::prelude::*;
use orkhon::ttensor::prelude::*;
use rand::*;
use std::path::PathBuf;

 let o = Orkhon::new()
     .config(
         OrkhonConfig::new()
             .with_input_fact_shape(InferenceFact::dt_shape(f32::datum_type(), tvec![10, 100])),
     )
     .onnx(
         "model_which_will_be_tested",
         PathBuf::from("tests/protobuf/onnx_model/example.onnx"),
     )
     .build();

 let mut rng = thread_rng();
 let vals: Vec<_> = (0..1000).map(|_| rng.gen::<f32>()).collect();
 let input = tract_ndarray::arr1(&vals).into_shape((10, 100)).unwrap();

 let resp = o
     .onnx_request(
         "model_which_will_be_tested",
         ORequest::with_body(ONNXRequest::new().body(input.into())),
     )
     .unwrap();
 assert_eq!(resp.body.output.len(), 1);
```

## License

License is [MIT](https://github.com/vertexclique/orkhon/blob/master/LICENSE)

## Documentation

Official documentation is hosted on [docs.rs](https://docs.rs/orkhon).

## Getting Help
Please head to our [Gitter](https://gitter.im/orkhonml/community) or use [StackOverflow](https://stackoverflow.com/questions/tagged/orkhon)

## Discussion and Development
We use [Gitter](https://gitter.im/orkhonml/community) for development discussions. Also please don't hesitate to open issues on GitHub ask for features, report bugs, comment on design and more!
More interaction and more ideas are better!

## Contributing to Orkhon [![Open Source Helpers](https://www.codetriage.com/vertexclique/orkhon/badges/users.svg)](https://www.codetriage.com/vertexclique/orkhon)

All contributions, bug reports, bug fixes, documentation improvements, enhancements and ideas are welcome.

A detailed overview on how to contribute can be found in the  [CONTRIBUTING guide](.github/CONTRIBUTING.md) on GitHub.
