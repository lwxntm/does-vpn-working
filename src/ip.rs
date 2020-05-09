use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Ip {
    #[serde(default = "default_field")]
    pub organization: String,
    #[serde(default = "default_field")]
    pub isp: String,
    #[serde(default = "default_field")]
    pub city: String,
    #[serde(default = "default_field")]
    pub country: String,
    #[serde(default = "default_field")]
    pub ip: String,
}

fn default_field() -> String {
    "~".to_string()
}
