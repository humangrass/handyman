use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Granularity {
    #[serde(rename = "1min")]
    OneMin,
    #[serde(rename = "5min")]
    FiveMin,
    #[serde(rename = "15min")]
    FifteenMin,
    #[serde(rename = "1h")]
    OneHour,
}

impl FromStr for Granularity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1min" => Ok(Granularity::OneMin),
            "5min" => Ok(Granularity::FiveMin),
            "15min" => Ok(Granularity::FifteenMin),
            "1h" => Ok(Granularity::OneHour),
            _ => Err(format!("Invalid granularity: {}", s)),
        }
    }
}

impl Granularity {
    pub fn as_millis(&self) -> i64 {
        match self {
            Granularity::OneMin => 60 * 1000,
            Granularity::FiveMin => 5 * 60 * 1000,
            Granularity::FifteenMin => 15 * 60 * 1000,
            Granularity::OneHour => 60 * 60 * 1000,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Granularity::OneMin => "1min",
            Granularity::FiveMin => "5min",
            Granularity::FifteenMin => "15min",
            Granularity::OneHour => "1h",
        }
    }
}
