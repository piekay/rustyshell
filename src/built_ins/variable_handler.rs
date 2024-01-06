use std::collections::HashMap;
use std::env;

pub(crate) fn get_value(var: String, map: HashMap<String, String>) -> String {
    for(key, value) in map {
        if key == var {
            return value;
        }
    }
    return "none".parse().unwrap();
}

pub(crate) fn set_vars(key: String, value: String, mut map: HashMap<String, String>) -> HashMap<String, String>{
    map.insert(key, value);
    map
}
pub(crate) fn get_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for (key, value) in env::vars() {
        vars.insert(key, value);
    }
    vars
}