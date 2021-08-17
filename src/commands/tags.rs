use crate::config::Tags;

use anyhow::Result;
use crate::csv::CsvLineReader;
use std::collections::HashMap;

pub fn tags(_tags_opts: &Tags, csv: &str) -> Result<()> {
    let map = get_tags(csv)?;

    let mut keys = map.keys().collect::<Vec<&String>>();
    keys.sort(); // Already lowercase
    for key in keys {
        let value = map.get(key.as_str());
        if let Some(tags) = value {
            let mut tags = tags.clone();
            tags.sort();
            println!("{}", tags.join(", "));
        }
    }

    Ok(())
}

fn get_tags(csv: &str) -> Result<HashMap<String, Vec<String>>> {
    let reader = CsvLineReader::new(csv)?;

    let mut map = HashMap::new();

    for line in reader {
        let line = line?;

        for tag in line.tags {
            let key = tag.to_lowercase();
            let entry = map.entry(key).or_insert_with(Vec::new);
            if !entry.contains(&tag) {
                entry.push(tag);
            }
        }
    }

    Ok(map)
}