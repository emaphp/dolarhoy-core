use nom::{
    bytes::complete::{tag, take_while},
    combinator::map_res,
    IResult,
};
use unhtml::{self, FromHtml};

pub const HTTP_RESPONSE_STATUS_OK: u32 = 200;
pub const HTTP_RESPONSE_STATUS_NOT_FOUND: u32 = 404;

#[derive(Debug, PartialEq)]
pub struct HTTPResponse {
    pub protocol: String,
    pub version: String,
    pub status: u32,
}

impl HTTPResponse {
    pub fn status_ok(&self) -> bool {
        self.status == HTTP_RESPONSE_STATUS_OK
    }

    pub fn status_not_found(&self) -> bool {
        self.status == HTTP_RESPONSE_STATUS_NOT_FOUND
    }
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn is_float(c: char) -> bool {
    c.is_numeric() || c == '.'
}

fn is_numeric(c: char) -> bool {
    c.is_numeric()
}

fn from_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

pub fn http_response(input: &str) -> IResult<&str, HTTPResponse> {
    let (input, protocol) = take_while(is_alpha)(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, version) = take_while(is_float)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, status) = map_res(take_while(is_numeric), from_int)(input)?;

    Ok((
        input,
        HTTPResponse {
            protocol: protocol.to_owned(),
            version: version.to_owned(),
            status,
        },
    ))
}

#[derive(Debug)]
pub struct CotizacionPrice {
    pub precio: f64,
}

impl FromHtml for CotizacionPrice {
    fn from_elements(select: unhtml::ElemIter) -> unhtml::Result<Self> {
        let elements: Vec<_> = select.collect();

        let el = elements.first().ok_or(unhtml::Error::TextParseError {
            text: String::from("content"),
            type_name: String::from("float"),
            err: String::from("element not found"),
        })?;

        let content = el.inner_html();
        let (_, valor): (&str, &str) = take_while::<_, _, nom::error::Error<&str>>(is_float)(
            content.as_str(),
        )
        .map_err(|_: nom::Err<_>| unhtml::Error::TextParseError {
            text: el.inner_html(),
            type_name: String::from("float"),
            err: String::from("parse error"),
        })?;

        let precio = valor
            .parse::<f64>()
            .map_err(|_| unhtml::Error::TextParseError {
                text: valor.to_string(),
                type_name: String::from("f64"),
                err: String::from("conversion error"),
            })?;

        Ok(Self { precio })
    }
}

pub type CompraVenta = (f64, Option<f64>);

pub trait PrecioCompraVenta {
    fn precio_compra_venta(&self) -> CompraVenta;
    fn title(&self) -> String;
}

#[derive(Debug, FromHtml)]
#[html(selector = ".container__data")]
pub struct CotizacionCompraVenta {
    #[html(selector = "h2.data__titulo", attr = "inner")]
    pub title: String,

    #[html(selector = "p:nth-child(1)")]
    pub precio_compra: CotizacionPrice,

    #[html(selector = "p:nth-child(2)")]
    pub precio_venta: CotizacionPrice,
}

impl PrecioCompraVenta for CotizacionCompraVenta {
    fn precio_compra_venta(&self) -> CompraVenta {
        (self.precio_compra.precio, Some(self.precio_venta.precio))
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

#[derive(Debug, FromHtml)]
#[html(selector = ".container__data")]
pub struct CotizacionValor {
    #[html(selector = "h2.data__titulo", attr = "inner")]
    pub title: String,

    #[html(selector = "p:nth-child(1)")]
    pub valor: CotizacionPrice,
}

impl PrecioCompraVenta for CotizacionValor {
    fn precio_compra_venta(&self) -> CompraVenta {
        (self.valor.precio, None)
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response_header() {
        assert_eq!(
            http_response("HTTP/1.1 301"),
            Ok((
                "",
                HTTPResponse {
                    protocol: String::from("HTTP"),
                    version: String::from("1.1"),
                    status: 301
                }
            ))
        );

        let (_, resp) = http_response("HTTP/1.1 301").unwrap();
        assert!(!resp.status_ok());
        assert!(!resp.status_not_found());

        assert_eq!(
            http_response("HTTP/1.1 404"),
            Ok((
                "",
                HTTPResponse {
                    protocol: String::from("HTTP"),
                    version: String::from("1.1"),
                    status: 404
                }
            ))
        );

        let (_, resp) = http_response("HTTP/1.1 404").unwrap();
        assert!(!resp.status_ok());
        assert!(resp.status_not_found());

        assert_eq!(
            http_response("HTTP/1.1 200"),
            Ok((
                "",
                HTTPResponse {
                    protocol: String::from("HTTP"),
                    version: String::from("1.1"),
                    status: 200
                }
            ))
        );

        let (_, resp) = http_response("HTTP/1.1 200").unwrap();
        assert!(resp.status_ok());
        assert!(!resp.status_not_found());
    }

    #[test]
    fn test_cotizacion_compraventa_parse() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Blue</h2>
            <div class="data__valores">
                <p>566.00<span>Compra</span></p>
                <p>571.00<span>Venta</span></p>
            </div>
        </div>
    "#;

        let cotizacion = CotizacionCompraVenta::from_html(content);
        assert!(cotizacion.is_ok());

        let cotizacion = cotizacion.unwrap();
        assert_eq!(cotizacion.title, "Dólar Blue");
        assert_eq!(cotizacion.precio_compra.precio, 566.00f64);
        assert_eq!(cotizacion.precio_venta.precio, 571.00f64);
    }

    #[test]
    fn test_cotizacion_compraventa_missing() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Blue</h2>
            <div class="data__valores">
            </div>
        </div>
    "#;

        let cotizacion = CotizacionCompraVenta::from_html(content);
        assert!(cotizacion.is_err());
        assert_eq!(
            cotizacion.err().unwrap().to_string(),
            "content cannot be parsed as float: element not found"
        );
    }

    #[test]
    fn test_cotizacion_compraventa_invalid() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Blue</h2>
            <div class="data__valores">
                <p>.<span>Compra</span></p>
                <p>.<span>Venta</span></p>
            </div>
        </div>
    "#;

        let cotizacion = CotizacionCompraVenta::from_html(content);
        assert!(cotizacion.is_err());
        assert_eq!(
            cotizacion.err().unwrap().to_string(),
            ". cannot be parsed as f64: conversion error"
        );
    }

    #[test]
    fn test_cotizacion_valor_parse() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Crypto</h2>
            <div class="data__valores">
                <p>29169.00<span>Valor</span></p>
            </div>
        </div>
    "#;

        let cotizacion = CotizacionValor::from_html(content);
        assert!(cotizacion.is_ok());

        let cotizacion = cotizacion.unwrap();
        assert_eq!(cotizacion.title, "Dólar Crypto");
        assert_eq!(cotizacion.valor.precio, 29169.00f64);
    }

    #[test]
    fn test_cotizacion_valor_missing() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Crypto</h2>
            <div class="data__valores">
            </div>
        </div>
    "#;

        let cotizacion = CotizacionValor::from_html(content);
        assert!(cotizacion.is_err());
        assert_eq!(
            cotizacion.err().unwrap().to_string(),
            "content cannot be parsed as float: element not found"
        );
    }

    #[test]
    fn test_cotizacion_valor_invalid() {
        let content = r#"
        <div class="container__data" style="text-align:center;width:100%">
            <h2 class="data__titulo">Dólar Crypto</h2>
            <div class="data__valores">
                <p>.<span>Valor</span></p>
            </div>
        </div>
    "#;

        let cotizacion = CotizacionValor::from_html(content);
        assert!(cotizacion.is_err());
        assert_eq!(
            cotizacion.err().unwrap().to_string(),
            ". cannot be parsed as f64: conversion error"
        );
    }
}
