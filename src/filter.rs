use crate::json::JsonValue;
use regex::Regex;

/*
.					Return the entire JSON value
.name				Access an object field
.a.b				Access a nested field
.[0]				Get the first array element
.[]					Iterate over elements
*/
pub fn apply_filter(filter: &str, json_val: JsonValue) -> Result<JsonValue, String> {
    let pattern = if let Ok(regex_p) = Regex::new(
        r"\A\.(?:(?P<path>[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)|\[(?P<index>[0-9]*)\])?\z",
    ) {
        regex_p
    } else {
        return Err(format!("failed to parse regex"));
    };

    let caps = if let Some(caps) = pattern.captures(filter) {
        caps
    } else {
        return Err(format!("invalid filter"));
    };

    if let Some(path) = caps.name("path") {
        let fields: Vec<&str> = path.as_str().split(".").collect();
        println!("fields: {:?}", fields);
        return get_val_by_fields(fields, json_val);
    } else if let Some(index) = caps.name("index") {
        if index.is_empty() {
            println!("whole array");
            return get_array(json_val);
        } else {
            println!("get value at: {}", index.as_str());
            return get_val_by_index(index.as_str(), json_val);
        }
    } else {
        println!("get entrie kjson data");
    }
    Ok(json_val)
}

fn get_val_by_fields(fields: Vec<&str>, json_val: JsonValue) -> Result<JsonValue, String> {
    let mut start =
        get_val_recur(fields[0], &json_val).ok_or_else(|| format!("value not found"))?;

    for field in &fields[1..] {
        start = get_val_recur(field, start).ok_or_else(|| format!("value not found"))?;
    }

    Ok(start.clone())
}

fn get_val_recur<'a>(field: &str, val: &'a JsonValue) -> Option<&'a JsonValue> {
    if let JsonValue::Object(obj) = val {
        obj.get(field)
    } else {
        None
    }
}

fn get_val_by_index(index: &str, val: JsonValue) -> Result<JsonValue, String> {
    if let JsonValue::Array(arr) = val {
        let idx = if let Ok(n) = index.parse::<usize>() {
            n
        } else {
            return Err(format!("invalid indx"));
        };

        if idx >= arr.len() {
            return Err(format!("index out of range"));
        }

        Ok(arr[idx].clone())
    } else {
        Err(format!("value is not json array"))
    }
}
fn get_array(val: JsonValue) -> Result<JsonValue, String> {
    if let JsonValue::Array(arr) = val {
        Ok(JsonValue::Array(arr.clone()))
    } else {
        Err(format!("value is not json array"))
    }
}
