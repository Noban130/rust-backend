use csv::ReaderBuilder;
use std::error::Error;


pub fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;

let mut x_values = Vec::new();
let mut y_values = Vec::new();

for (i, result) in reader.records().enumerate() {
    match result {
        Ok(record) => {
            println!("Record {}: {:?}", i + 1, record);
            let x: f64 = match record[0].parse() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error parsing 'x' at record {}: {}", i + 1, e);
                    continue;  // Skip this record
                }
            };
            let y: f64 = match record[1].parse() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error parsing 'y' at record {}: {}", i + 1, e);
                    continue;  // Skip this record
                }
            };
            x_values.push(x);
            y_values.push(y);
        }
        Err(e) => {
            eprintln!("Error reading record {}: {}", i + 1, e);
        }
    }
}

// println!("Parsed x values: {:?}", x_values);
// println!("Parsed y values: {:?}", y_values);

Ok((x_values, y_values))
}
