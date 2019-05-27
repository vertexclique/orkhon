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
    <a href="https://travis-ci.org/vertexclique/orkhon">
    <img src="https://travis-ci.org/vertexclique/orkhon.svg?branch=master" alt="travis build status" />
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

## Installation

You can include Orkhon into your project with;

```toml
[dependencies]
orkhon = "0.1.0"
```

## Dependencies
You will need:
* Rust Nightly needed (for now. until async support fully lands in)
* Python dev dependencies installed and have proper python runtime to use Orkhon with your project.
* Point out your `PYTHONHOME` environment variable to your Python installation.

## Python API contract

For Python API contract you can take a look at the [Project Documentation](https://docs.rs/orkhon).

## Examples
#### Minimal Async Model Request Example

```rust
let o = Orkhon::new()
    .config(OrkhonConfig::new())
    .pymodel("model_which_will_be_tested", // Unique identifier of the model
             "tests/pymodels",             // Python module directory
             "model_test",                 // Python module file name
        "model_hook"                       // Hook(Python method) that will be called by Orkhon
    )
    .build();

// Args for the request hook
let mut request_args = HashMap::new();
request_args.insert("is", 10);
request_args.insert("are", 6);
request_args.insert("you", 5);

// Kwargs for the request hook
let mut request_kwargs = HashMap::<&str, &str>::new();

// Future handle
let handle =
    o.pymodel_request_async(
        "model_which_will_be_tested",
        ORequest::with_body(
            PyModelRequest::new()
                .with_args(request_args)
                .with_kwargs(request_kwargs)
        )
    );

// Return the result
handle.await.unwrap()
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
