A crate for fetching currency values from [dolarhoy.com](https://dolarhoy.com/).

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
    let result = client.fetch_cotizacion(dolar::Cotizacion::Blue).await?;

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
