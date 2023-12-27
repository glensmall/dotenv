use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = ".env";

fn main() {

    let conf: HashMap<String, String> = HashMap::new();

    // open the file
    let contents = fs::read_to_string(FILE_PATH).expect("failed to read the file");

    let parts = contents.split("\n");

    for part in parts{
        let kv = part.split("=");

        // add the kv to the hashmap
    }
    
}
