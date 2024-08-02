use serde::Serialize;
use crate::config::config::Config;

pub fn to_string<T>(value: &T) -> serenity::Result<String>
where
    T: ?Sized + Serialize,
{
    let is_json_pretty = Config::new().is_json_pretty;
    let result = if is_json_pretty {
        serenity::json::to_string_pretty(value)?
    } else {
        serenity::json::to_string(value)?
    };
    Ok(result)
}