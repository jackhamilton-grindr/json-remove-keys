use std::{env, fs, path::Path};

use json::JsonValue;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[2] == "-h" {
        println!("Error: not enough information provided. Command should be formatted: json-filter [filename] [key 1] [key 2] [etc].
All such keys will be removed at all levels. Keys are case-sensitive.
Note: please make sure this is what you want. For example, there could be multiple fields at different levels called, for instance, \"name\".
Keys with spaces should be surrounded with quotes, for example: json-filter [filename] \"[key name]\"");
        return
    }
    let filepath = Path::new(&args[1]);
    let contents = fs::read_to_string(filepath).expect("Could not read file!");
    let parsed = json::parse(&contents).expect("Error: File is not valid JSON.");
    let mut removals = args.clone();

    removals.drain(0..2);
    // args now contains removals
    let output = recursive_blast(parsed, &removals);
    let _ = fs::write(filepath, output.dump());
}

fn recursive_blast(mut current_state: JsonValue, removals: &Vec<String>) -> JsonValue {
    for key in current_state.clone().entries() {
        if let JsonValue::Object(obj) = key.1.clone() {
            let str_representation = obj.dump();
            let json_encoding = json::parse(&str_representation).expect("Error: File contains an object which could not be parsed.");
            current_state[key.0] = recursive_blast(json_encoding, removals);
        };
        if removals.contains(&key.0.to_string()) {
            current_state.remove(key.0);
        }
    }
    current_state
}
