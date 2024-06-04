use std::fs::File;
use tempfile::TempDir;
use serde_json::{json, Value};
fn main(){
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let unique_id = 0348293048;
    let mut file = File::create(unique_id.to_string()).expect("Could not create file!");
    let value = json!({
        "previous": unique_id,
        "command": "set",
        "key": "value",
        "value": "value",
    });
    file.write_all(value.as_bytes())
        .expect("Cannot write to the file!");
}