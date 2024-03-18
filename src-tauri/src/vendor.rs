use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}, path::{Path, PathBuf}};

pub fn get_vendors(path: PathBuf) -> Result<HashMap<String, String>, io::Error> {
    let mut map : HashMap<String, String> = HashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line?;
        let split: Vec<&str> = line.split(" ").collect();
        map.insert(split[0].to_string(), split[1].to_string());
    }

    Ok(map)
}