use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();

    let schema_file = File::open(&args[1]).unwrap();
    let schema_value = serde_json::from_reader(schema_file).unwrap();
    let schema = JSONSchema::compile(&schema_value, Some(Draft::Draft7)).unwrap();

    let test_file = File::open(&args[2]).unwrap();
    let test_value: Value = serde_json::from_reader(test_file).unwrap();

    let interchange_value = test_value.get("interchange").unwrap();

    match schema.validate(interchange_value) {
        Ok(()) => println!("Validated successfully"),
        Err(errors) => {
            for e in errors {
                println!("Error: {:?}", e);
            }
        }
    };
}
