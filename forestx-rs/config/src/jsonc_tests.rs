use super::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Sample {
    name: String,
    count: u32,
}

#[test]
fn parses_line_comments() {
    let input = r#"{
  "name": "forestx", // user-facing label
  "count": 42
}"#;

    let parsed: Sample = parse_jsonc(input).expect("line comments should parse");

    assert_eq!(
        parsed,
        Sample {
            name: "forestx".into(),
            count: 42,
        }
    );
}

#[test]
fn parses_block_comments_and_trailing_comma() {
    let input = r#"{
  /* block comment */ "name": "x",
  "count": 7,
}"#;

    let parsed: Sample = parse_jsonc(input).expect("block comment and trailing comma should parse");

    assert_eq!(
        parsed,
        Sample {
            name: "x".into(),
            count: 7,
        }
    );
}

#[test]
fn pretty_roundtrips() {
    let value = Sample {
        name: "x".into(),
        count: 1,
    };

    let serialized = to_jsonc_pretty(&value).expect("serialize");
    let back: Sample = parse_jsonc(&serialized).expect("parse back");

    assert_eq!(back, value);
}

#[test]
fn converts_primitives() {
    assert_eq!(
        json_value_to_toml(json!(true)).unwrap(),
        toml::Value::Boolean(true)
    );
    assert_eq!(
        json_value_to_toml(json!(42)).unwrap(),
        toml::Value::Integer(42)
    );
    assert_eq!(
        json_value_to_toml(json!(3.5)).unwrap(),
        toml::Value::Float(3.5)
    );
    assert_eq!(
        json_value_to_toml(json!("hi")).unwrap(),
        toml::Value::String("hi".into())
    );
}

#[test]
fn converts_nested_arrays_and_tables() {
    let value = json!({
        "nums": [1, 2, 3],
        "nested": { "flag": true }
    });

    let toml_value = json_value_to_toml(value).unwrap();

    assert_eq!(
        toml_value["nums"].as_array(),
        Some(&vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
            toml::Value::Integer(3),
        ])
    );
    assert_eq!(toml_value["nested"]["flag"].as_bool(), Some(true));
}

#[test]
fn drops_null_object_fields() {
    // null fields are dropped so Option<T> deserializes to None, mirroring TOML
    // where an absent key is the only spelling of "none".
    let toml_value = json_value_to_toml(json!({"a": 1, "b": null})).unwrap();
    let table = toml_value.as_table().unwrap();

    assert!(table.contains_key("a"));
    assert!(!table.contains_key("b"));
}

#[test]
fn rejects_null_at_top_level_and_in_arrays() {
    assert!(matches!(
        json_value_to_toml(json!(null)).unwrap_err(),
        JsoncConfigError::UnsupportedValue(_)
    ));
    assert!(matches!(
        json_value_to_toml(json!([1, null])).unwrap_err(),
        JsoncConfigError::UnsupportedValue(_)
    ));
}

#[test]
fn parse_jsonc_as_toml_keeps_comments_out_and_values_in() {
    let src = r#"{
  // user model choice
  "model": "deepseek-v4-flash",
  "model_provider": "deepseek",
  "auto_review_model_override": null,
  "features": { "code_mode_host": true, }
}"#;

    let toml_value = parse_jsonc_as_toml(src).expect("jsonc -> toml");

    assert_eq!(toml_value["model"].as_str(), Some("deepseek-v4-flash"));
    assert_eq!(toml_value["model_provider"].as_str(), Some("deepseek"));
    assert_eq!(
        toml_value["features"]["code_mode_host"].as_bool(),
        Some(true)
    );
    // null field dropped -> key absent
    assert!(
        !toml_value
            .as_table()
            .unwrap()
            .contains_key("auto_review_model_override")
    );
}
