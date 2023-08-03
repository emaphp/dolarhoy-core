pub const DOLAR_HOY_DOMAIN: &str = "dolarhoy.com";
pub const DOLAR_HOY_ENDPOINT_BASE: &str = "/i/cotizaciones/";

pub const DOLAR_HOY_RESOURCE_BLUE: &str = "dolar-blue";
pub const DOLAR_HOY_RESOURCE_OFICIAL: &str = "dolar-bancos-y-casas-de-cambio";
pub const DOLAR_HOY_RESOURCE_BOLSA: &str = "dolar-mep";
pub const DOLAR_HOY_RESOURCE_CCL: &str = "dolar-contado-con-liquidacion";
pub const DOLAR_HOY_RESOURCE_CRYPTO: &str = "bitcoin-usd";
pub const DOLAR_HOY_RESOURCE_SOLIDARIO: &str = "banco-nacion";

pub const DOLAR_HOY_ALIAS_BLUE: &[&str] = &[DOLAR_HOY_RESOURCE_BLUE, "blue"];
pub const DOLAR_HOY_ALIAS_OFICIAL: &[&str] = &[DOLAR_HOY_RESOURCE_OFICIAL, "oficial"];
pub const DOLAR_HOY_ALIAS_BOLSA: &[&str] = &[DOLAR_HOY_RESOURCE_BOLSA, "bolsa", "mep"];
pub const DOLAR_HOY_ALIAS_CCL: &[&str] = &[DOLAR_HOY_RESOURCE_CCL, "ccl", "contado"];
pub const DOLAR_HOY_ALIAS_CRYPTO: &[&str] =
    &[DOLAR_HOY_RESOURCE_CRYPTO, "crypto", "cripto", "bitcoin"];
pub const DOLAR_HOY_ALIAS_SOLIDARIO: &[&str] = &[DOLAR_HOY_RESOURCE_SOLIDARIO, "solidario", "bna"];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Moneda {
    ARS,
    USD,
}

impl ToString for Moneda {
    fn to_string(&self) -> String {
        match self {
            Self::ARS => String::from("Peso Argentino"),
            Self::USD => String::from("Dolar Estadounidense"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cotizacion {
    Blue,
    Oficial,
    Bolsa,
    ContadoConLiqui,
    Crypto,
    Solidario,
}

impl Cotizacion {
    pub fn endpoint(&self) -> String {
        format!(
            "{}{}",
            DOLAR_HOY_ENDPOINT_BASE,
            match self {
                Self::Blue => DOLAR_HOY_RESOURCE_BLUE,
                Self::Oficial => DOLAR_HOY_RESOURCE_OFICIAL,
                Self::Bolsa => DOLAR_HOY_RESOURCE_BOLSA,
                Self::ContadoConLiqui => DOLAR_HOY_RESOURCE_CCL,
                Self::Crypto => DOLAR_HOY_RESOURCE_CRYPTO,
                Self::Solidario => DOLAR_HOY_RESOURCE_SOLIDARIO,
            }
        )
    }

    pub fn moneda(&self) -> Moneda {
        match self {
            Self::Crypto => Moneda::USD,
            _ => Moneda::ARS,
        }
    }
}

impl ToString for Cotizacion {
    fn to_string(&self) -> String {
        match self {
            Self::Blue => String::from("Blue"),
            Self::Oficial => String::from("Oficial"),
            Self::Bolsa => String::from("Bolsa"),
            Self::ContadoConLiqui => String::from("Contado Con Liqui"),
            Self::Crypto => String::from("Crypto"),
            Self::Solidario => String::from("Solidario"),
        }
    }
}

pub fn get_cotizacion_from_resource_name(name: &str) -> Option<Cotizacion> {
    match name {
        DOLAR_HOY_RESOURCE_BLUE => return Some(Cotizacion::Blue),
        DOLAR_HOY_RESOURCE_OFICIAL => return Some(Cotizacion::Oficial),
        DOLAR_HOY_RESOURCE_BOLSA => return Some(Cotizacion::Bolsa),
        DOLAR_HOY_RESOURCE_CCL => return Some(Cotizacion::ContadoConLiqui),
        DOLAR_HOY_RESOURCE_CRYPTO => return Some(Cotizacion::Crypto),
        DOLAR_HOY_RESOURCE_SOLIDARIO => return Some(Cotizacion::Solidario),
        _ => return None,
    }
}

pub fn get_cotizacion_from_alias(alias: &str) -> Option<Cotizacion> {
    if let Some(_) = DOLAR_HOY_ALIAS_BLUE.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::Blue);
    }

    if let Some(_) = DOLAR_HOY_ALIAS_OFICIAL.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::Oficial);
    }

    if let Some(_) = DOLAR_HOY_ALIAS_BOLSA.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::Bolsa);
    }

    if let Some(_) = DOLAR_HOY_ALIAS_CCL.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::ContadoConLiqui);
    }

    if let Some(_) = DOLAR_HOY_ALIAS_CRYPTO.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::Crypto);
    }

    if let Some(_) = DOLAR_HOY_ALIAS_SOLIDARIO.into_iter().find(|&s| *s == alias) {
        return Some(Cotizacion::Solidario);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moneda_cotizacion() {
        assert_eq!(Cotizacion::Blue.moneda(), Moneda::ARS);
        assert_eq!(Cotizacion::Oficial.moneda(), Moneda::ARS);
        assert_eq!(Cotizacion::Bolsa.moneda(), Moneda::ARS);
        assert_eq!(Cotizacion::ContadoConLiqui.moneda(), Moneda::ARS);
        assert_eq!(Cotizacion::Crypto.moneda(), Moneda::USD);
        assert_eq!(Cotizacion::Solidario.moneda(), Moneda::ARS);
    }

    #[test]
    fn test_cotizacion_from_name() {
        assert_eq!(get_cotizacion_from_resource_name(""), None);
        assert_eq!(
            get_cotizacion_from_resource_name("dolar-blue"),
            Some(Cotizacion::Blue)
        );
        assert_eq!(
            get_cotizacion_from_resource_name("dolar-bancos-y-casas-de-cambio"),
            Some(Cotizacion::Oficial)
        );
        assert_eq!(
            get_cotizacion_from_resource_name("dolar-mep"),
            Some(Cotizacion::Bolsa)
        );
        assert_eq!(
            get_cotizacion_from_resource_name("dolar-contado-con-liquidacion"),
            Some(Cotizacion::ContadoConLiqui)
        );
        assert_eq!(
            get_cotizacion_from_resource_name("bitcoin-usd"),
            Some(Cotizacion::Crypto)
        );
        assert_eq!(
            get_cotizacion_from_resource_name("banco-nacion"),
            Some(Cotizacion::Solidario)
        );
    }

    #[test]
    fn test_get_cotizacion_endpoint() {
        assert_eq!(Cotizacion::Blue.endpoint(), "/i/cotizaciones/dolar-blue");
        assert_eq!(
            Cotizacion::Oficial.endpoint(),
            "/i/cotizaciones/dolar-bancos-y-casas-de-cambio"
        );
        assert_eq!(Cotizacion::Bolsa.endpoint(), "/i/cotizaciones/dolar-mep");
        assert_eq!(
            Cotizacion::ContadoConLiqui.endpoint(),
            "/i/cotizaciones/dolar-contado-con-liquidacion"
        );
        assert_eq!(Cotizacion::Crypto.endpoint(), "/i/cotizaciones/bitcoin-usd");
        assert_eq!(
            Cotizacion::Solidario.endpoint(),
            "/i/cotizaciones/banco-nacion"
        );
    }

    #[test]
    fn test_get_cotizacion_from_alias_blue() {
        assert_eq!(
            get_cotizacion_from_alias("dolar-blue"),
            Some(Cotizacion::Blue)
        );
        assert_eq!(get_cotizacion_from_alias("blue"), Some(Cotizacion::Blue));
    }

    #[test]
    fn test_get_cotizacion_from_alias_oficial() {
        assert_eq!(
            get_cotizacion_from_alias("dolar-bancos-y-casas-de-cambio"),
            Some(Cotizacion::Oficial)
        );
        assert_eq!(
            get_cotizacion_from_alias("oficial"),
            Some(Cotizacion::Oficial)
        );
    }

    #[test]
    fn test_get_cotizacion_from_alias_bolsa() {
        assert_eq!(
            get_cotizacion_from_alias("dolar-mep"),
            Some(Cotizacion::Bolsa)
        );
        assert_eq!(get_cotizacion_from_alias("bolsa"), Some(Cotizacion::Bolsa));
        assert_eq!(get_cotizacion_from_alias("mep"), Some(Cotizacion::Bolsa));
    }

    #[test]
    fn test_get_cotizacion_from_alias_ccl() {
        assert_eq!(
            get_cotizacion_from_alias("dolar-contado-con-liquidacion"),
            Some(Cotizacion::ContadoConLiqui)
        );
        assert_eq!(
            get_cotizacion_from_alias("ccl"),
            Some(Cotizacion::ContadoConLiqui)
        );
        assert_eq!(
            get_cotizacion_from_alias("contado"),
            Some(Cotizacion::ContadoConLiqui)
        );
    }

    #[test]
    fn test_get_cotizacion_from_alias_crypto() {
        assert_eq!(
            get_cotizacion_from_alias("bitcoin-usd"),
            Some(Cotizacion::Crypto)
        );
        assert_eq!(
            get_cotizacion_from_alias("crypto"),
            Some(Cotizacion::Crypto)
        );
        assert_eq!(
            get_cotizacion_from_alias("cripto"),
            Some(Cotizacion::Crypto)
        );
        assert_eq!(
            get_cotizacion_from_alias("bitcoin"),
            Some(Cotizacion::Crypto)
        );
    }

    #[test]
    fn test_get_cotizacion_from_alias_solidario() {
        assert_eq!(
            get_cotizacion_from_alias("banco-nacion"),
            Some(Cotizacion::Solidario)
        );
        assert_eq!(
            get_cotizacion_from_alias("solidario"),
            Some(Cotizacion::Solidario)
        );
        assert_eq!(
            get_cotizacion_from_alias("bna"),
            Some(Cotizacion::Solidario)
        );
    }
}
