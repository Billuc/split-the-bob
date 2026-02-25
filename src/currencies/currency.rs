use serde_json::{Map, value::Value};
use std::{env, fmt::Display, ops::Deref};

#[derive(Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub country: String,
    pub country_code: Option<String>
}

#[derive(Debug)]
pub enum CurrencyError {
    ReqwestError(reqwest::Error),
    MissingEnvVar(String),
    IncorrectAPIData,
    APIError(String),
    UnknownCurrency(String),
}

impl Display for CurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyError::ReqwestError(error) => write!(f, "{}", error),
            CurrencyError::MissingEnvVar(envvar) => write!(f, "Missing env var: {}", envvar),
            CurrencyError::IncorrectAPIData => write!(f, "Incorrect API data"),
            CurrencyError::APIError(err_str) => write!(f, "API error: {}", err_str),
            CurrencyError::UnknownCurrency(curr) => write!(f, "Unknown currency: {}", curr),
        }
    }
}

pub const CURRENCIES_CSV: &str = include_str!("currencies.csv");
const TOKEN_ENV_VAR: &str = "EXCHANGE_RATE_API_KEY";

lazy_static::lazy_static! {
    pub static ref CURRENCIES: Vec<Currency> = CURRENCIES_CSV
        .lines()
        .skip(1)
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Currency {
                code: parts[0].trim().to_string(),
                name: parts[1].trim().to_string(),
                country: parts[2].trim().to_string(),
                country_code: parts.get(3).map(|code| code.trim().to_string()),
            }
        })
        .collect();

    static ref API_KEY: Option<String> = env::var(TOKEN_ENV_VAR).ok();
}

pub fn try_get_currency(code: &str) -> Result<&Currency, CurrencyError> {
    let currency = CURRENCIES
        .iter()
        .find(|c| c.code.to_uppercase() == code.to_uppercase());

    match currency {
        Some(curr) => Ok(curr),
        None => Err(CurrencyError::UnknownCurrency(code.to_string())),
    }
}

pub async fn convert(amount: f32, from: &Currency, to: &Currency) -> Result<f32, CurrencyError> {
    let token = match API_KEY.deref() {
        Some(t) => t,
        None => return Err(CurrencyError::MissingEnvVar(TOKEN_ENV_VAR.to_string()))
    };

    let endpoint = format!(
        "https://v6.exchangerate-api.com/v6/{}/pair/{}/{}/{:.2}",
        token, from.code, to.code, amount
    );

    let request = reqwest::get(endpoint)
        .await
        .map_err(CurrencyError::ReqwestError)?;
    let result: Value = request
        .json::<Value>()
        .await
        .map_err(CurrencyError::ReqwestError)?;

    parse_result(result)
}

fn parse_result(value: Value) -> Result<f32, CurrencyError> {
    if let Value::Object(map) = value {
        match map.get("result") {
            Some(Value::String(str)) => match str.as_str() {
                "success" => parse_success_message(map),
                "error" => parse_error_message(map),
                _ => return Err(CurrencyError::IncorrectAPIData),
            },
            _ => return Err(CurrencyError::IncorrectAPIData),
        }
    } else {
        return Err(CurrencyError::IncorrectAPIData);
    }
}

fn parse_success_message(object: Map<String, Value>) -> Result<f32, CurrencyError> {
    match object.get("conversion_result") {
        Some(Value::Number(val)) => match val.as_f64() {
            Some(value) => Ok(value as f32),
            None => Err(CurrencyError::IncorrectAPIData),
        },
        _ => Err(CurrencyError::IncorrectAPIData),
    }
}

fn parse_error_message(object: Map<String, Value>) -> Result<f32, CurrencyError> {
    match object.get("error-type") {
        Some(Value::String(str)) => Err(CurrencyError::APIError(str.clone())),
        _ => Err(CurrencyError::IncorrectAPIData),
    }
}
