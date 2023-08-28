A crate for fetching currency values from [dolarhoy.com](https://dolarhoy.com/).

[![dolarhoy-core on Crates.io](https://img.shields.io/crates/v/dolarhoy-core.svg?color=brightgreen)](https://crates.io/crates/dolarhoy-core)
[![Documentation](https://img.shields.io/docsrs/dolarhoy-core/latest.svg)](https://docs.rs/dolarhoy_core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/emaphp/dolarhoy-core/blob/master/LICENSE)
----------------

Table of Contents
=================

* [Basic Usage](#basic-usage)
* [License](#license)
* [Disclaimer](#disclaimer)

### Basic Usage

```rust
use dolarhoy_core::{client, dolar, error};

type Result<T> = std::result::Result<T, error::ClientError>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::DolayHoyClient::new();
    let result = client.fetch_cotizacion::<f32>(dolar::Cotizacion::Blue).await?;

    match result.precio_compra_venta() {
        (compra, None) => println!("{}: {}", result.title(), compra),
        (compra, Some(venta)) => println!("{}: {} / {}", result.title(), compra, venta),
    }

    Ok(())
}
```

### License

Released under the MIT License.

### Disclaimer

DolarHoy.com ® is a registered trademark. I don't hold any type of relation to the company or its staff.
