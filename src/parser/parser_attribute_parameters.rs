use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref PARSER_OPTIONS: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();

        hash_map.insert("clip t", "00:00:30");

        hash_map.insert(
            "parser about",
            "Media tool for clipping videos, extracting \
            frames, and other features.",
        );

        hash_map
    };
}

macro_rules! attribute_parameter {
    ($option:expr) => {
        PARSER_OPTIONS.get($option).unwrap().to_owned()
    };
}

pub(crate) use attribute_parameter;
