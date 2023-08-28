use crate::dolar::{self, Cotizacion};
use crate::error::ClientError;
use crate::parser;
use std::io;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{self, OwnedTrustAnchor};
use tokio_rustls::TlsConnector;
use unhtml::FromHtml;

/// A client for fetching currency prices
pub struct DolayHoyClient {}

impl DolayHoyClient {
    /// Returns a new DolayHoyClient instance
    pub fn new() -> Self {
        DolayHoyClient {}
    }

    /// Fetches the prices of the given currency
    /// Returns a Result wrapping a Box<dyn parser::PrecioCompraVenta<T>>
    /// Where T could be either be f32 o f64
    ///
    /// # Example
    ///
    /// ```
    /// use dolarhoy_core::{client, dolar, error};
    ///
    /// type Result<T> = std::result::Result<T, error::ClientError>;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let client = client::DolayHoyClient::new();
    ///   let result = client.fetch_cotizacion::<f32>(dolar::Cotizacion::Oficial).await;
    ///
    ///   match result {
    ///     Err(e) => { panic!("{}", e) },
    ///     Ok(price) => {
    ///       let buy_sell_price = price.precio_compra_venta();
    ///       println!("Buy: {} / Sale: {}", buy_sell_price.0, buy_sell_price.1.unwrap());
    ///     },
    ///   }
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn fetch_cotizacion<T: Send + Copy + FromStr + 'static>(
        &self,
        cotizacion: dolar::Cotizacion,
    ) -> Result<Box<dyn parser::PrecioCompraVenta<T>>, ClientError> {
        use ClientError::*;

        let addr = (dolar::DOLAR_HOY_DOMAIN, 443)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

        let domain = dolar::DOLAR_HOY_DOMAIN;
        let uri = cotizacion.endpoint();
        let content = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", uri, domain);

        let mut root_cert_store = rustls::RootCertStore::empty();
        root_cert_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS[..].iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
        let connector = TlsConnector::from(Arc::new(config));
        let stream = TcpStream::connect(&addr).await?;

        let domain = rustls::ServerName::try_from(domain)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

        let mut stream = connector.connect(domain, stream).await?;
        stream.write_all(content.as_bytes()).await?;

        let (mut reader, _) = split(stream);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;

        let res = String::from_utf8(buffer).map_err(|e| InvalidResponseError(e.to_string()))?;

        match &res.find("\r\n\r\n") {
            None => {
                return Err(InvalidResponseError("invalid content".to_string()));
            }
            Some(s) => {
                let (_, response) = parser::http_response(&res.as_str()[..(*s)])
                    .map_err(|e| InvalidResponseError(e.to_string()))?;

                if !response.status_ok() {
                    return Err(ResponseStatusError(response.status));
                }

                let content = &res.as_str()[(*s + 4)..];

                match cotizacion {
                    Cotizacion::Crypto => {
                        return Ok(Box::new(parser::CotizacionValor::from_html(content)?))
                    }
                    _ => return Ok(Box::new(parser::CotizacionCompraVenta::from_html(content)?)),
                }
            }
        }
    }
}
