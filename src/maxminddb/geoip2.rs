/// GeoIP2 Country record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Country {
    pub continent: Option<model::Continent>,
    pub country: Option<model::Country>,
    pub registered_country: Option<model::Country>,
    pub represented_country: Option<model::RepresentedCountry>,
    pub traits: Option<model::Traits>,
}

/// GeoIP2 City record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct City {
    pub city: Option<model::City>,
    pub continent: Option<model::Continent>,
    pub country: Option<model::Country>,
    pub location: Option<model::Location>,
    pub postal: Option<model::Postal>,
    pub registered_country: Option<model::Country>,
    pub represented_country: Option<model::RepresentedCountry>,
    pub subdivisions: Option<Vec<model::Subdivision>>,
    pub traits: Option<model::Traits>,
}

/// GeoIP2 ISP record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Isp {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
}

/// GeoIP2 Connection-Type record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConnectionType {
    pub connection_type: Option<String>,
}

/// GeoIP2 Anonymous Ip record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AnonymousIp {
    pub is_anonymous: Option<bool>,
    pub is_anonymous_vpn: Option<bool>,
    pub is_hosting_provider: Option<bool>,
    pub is_public_proxy: Option<bool>,
    pub is_tor_exit_node: Option<bool>,
}

/// GeoIP2 DensityIncome record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DensityIncome {
    pub average_income: Option<u32>,
    pub population_density: Option<u32>,
}

/// GeoIP2 Domain record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Domain {
    pub domain: Option<String>,
}

/// GeoIP2 Asn record
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Asn {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
}

pub mod model {
    use std::collections::BTreeMap;

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct City {
        pub geoname_id: Option<u32>,
        pub names: Option<BTreeMap<String, String>>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Continent {
        pub code: Option<String>,
        pub geoname_id: Option<u32>,
        pub names: Option<BTreeMap<String, String>>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Country {
        pub geoname_id: Option<u32>,
        pub is_in_european_union: Option<bool>,
        pub iso_code: Option<String>,
        pub names: Option<BTreeMap<String, String>>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Location {
        pub latitude: Option<f64>,
        pub longitude: Option<f64>,
        pub metro_code: Option<u16>,
        pub time_zone: Option<String>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Postal {
        pub code: Option<String>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct RepresentedCountry {
        pub geoname_id: Option<u32>,
        pub iso_code: Option<String>,
        pub names: Option<BTreeMap<String, String>>, // pub type: Option<String>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Subdivision {
        pub geoname_id: Option<u32>,
        pub iso_code: Option<String>,
        pub names: Option<BTreeMap<String, String>>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Traits {
        pub is_anonymous_proxy: Option<bool>,
        pub is_satellite_provider: Option<bool>,
    }
}
