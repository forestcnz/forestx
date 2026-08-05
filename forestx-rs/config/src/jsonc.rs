//! JSONC (JSON with comments) serialization helpers for forestx config files.
//!
//! Built on [`serde_json_lenient`], which accepts C-style `//` and `/* */`
//! comments and trailing commas. This preserves the comment ergonomics users
//! previously had with TOML config files.
//!
//! The on-disk format is JSONC, but the rest of the config crate operates on
//! [`toml::Value`] (`TomlValue`). [`parse_jsonc_as_toml`] bridges the two: it
//! parses a JSONC document and converts it into a `toml::Value`, so the
//! existing loader / merge / diagnostics pipeline is unchanged.
//!
//! Programmatic serialization always emits plain pretty JSON (valid JSONC);
//! human-authored comments are not retained across rewrites, matching the
//! prior TOML behavior where round-tripping through `toml::Value` also dropped
//! comments.

use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

/// Parse a JSONC document (comments and trailing commas allowed) into `T`.
pub fn parse_jsonc<T: DeserializeOwned>(s: &str) -> Result<T, serde_json_lenient::Error> {
    serde_json_lenient::from_str(s)
}

// Serialize `T` as pretty-printed JSONC (plain pretty JSON).
//
// Not yet wired into the write path (managed-config upserts). Kept ready for the
// TOML->JSONC write boundary; remove the allow once a call site lands.
#[allow(dead_code)]
pub fn to_jsonc_pretty<T: Serialize>(value: &T) -> Result<String, serde_json_lenient::Error> {
    serde_json_lenient::to_string_pretty(value)
}

/// Errors raised while turning a JSONC document into a [`toml::Value`].
#[derive(Debug, Error)]
pub enum JsoncConfigError {
    /// The document is not valid JSONC.
    #[error("failed to parse JSONC config: {0}")]
    Parse(#[from] serde_json_lenient::Error),
    /// A JSON value with no TOML equivalent (for example `null` outside an
    /// object field).
    #[error("JSONC value has no TOML equivalent: {0}")]
    UnsupportedValue(String),
}

/// Parse a JSONC document and convert it into a [`toml::Value`].
///
/// This is the read boundary used by the config loader: the on-disk file is
/// JSONC, but downstream merging, diagnostics, and relative-path resolution
/// all work on `toml::Value`, so we convert at the edge.
pub fn parse_jsonc_as_toml(contents: &str) -> Result<toml::Value, JsoncConfigError> {
    let json: serde_json::Value = parse_jsonc(contents)?;
    json_value_to_toml(json)
}

/// Convert a [`serde_json::Value`] into the equivalent [`toml::Value`].
///
/// TOML has no null, so `null` is dropped when it appears as an object field
/// (equivalent to the field being absent, which deserializes `Option<T>` to
/// `None`); `null` anywhere else (top level or inside an array) is rejected.
pub(crate) fn json_value_to_toml(
    value: serde_json::Value,
) -> Result<toml::Value, JsoncConfigError> {
    use serde_json::Value;
    Ok(match value {
        Value::Bool(b) => toml::Value::Boolean(b),
        Value::Number(n) => json_number_to_toml(&n),
        Value::String(s) => toml::Value::String(s),
        Value::Array(items) => toml::Value::Array(
            items
                .into_iter()
                .map(json_value_to_toml)
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(map) => {
            let mut table = toml::map::Map::new();
            for (key, val) in map {
                if matches!(val, Value::Null) {
                    continue;
                }
                table.insert(key, json_value_to_toml(val)?);
            }
            toml::Value::Table(table)
        }
        Value::Null => {
            return Err(JsoncConfigError::UnsupportedValue(
                "null where TOML has no representation".into(),
            ));
        }
    })
}

fn json_number_to_toml(n: &serde_json::Number) -> toml::Value {
    if let Some(i) = n.as_i64() {
        toml::Value::Integer(i)
    } else if let Some(u) = n.as_u64() {
        // toml::Value::Integer is i64; values exceeding i64::MAX are exceedingly
        // unlikely in config files — fall back to f64 rather than failing.
        i64::try_from(u).map_or_else(|_| toml::Value::Float(u as f64), toml::Value::Integer)
    } else {
        toml::Value::Float(n.as_f64().unwrap_or(f64::NAN))
    }
}

#[cfg(test)]
#[path = "jsonc_tests.rs"]
mod tests;
