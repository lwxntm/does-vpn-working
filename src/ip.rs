use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Ip {
    pub organization: String,
    pub isp: String,
    #[serde(default)]
    #[serde(with = "date_serde")]
    pub city: String,
    pub country: String,
    pub ip: String,
}

#[test]
fn t() {
    let json: Ip = reqwest::blocking::get("https://api.ip.sb/geoip/185.209.84.53")
        .unwrap()
        .json()
        .unwrap();
    println!("{}", json.ip);
}
