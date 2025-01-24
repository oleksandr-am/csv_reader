use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use csv::ReaderBuilder;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);

    let mut obj = HashMap::new();
   
    for result in rdr.records() {
        let record = result?;

        let city_name = record.get(0).unwrap().to_string();
        let temp_value: f64 = record.get(1).unwrap().parse().expect("Failed to parse number");

        obj.entry(city_name)
            .or_insert_with(Vec::new)
            .push(temp_value);
        
        // TODO: read line and store intermediate results in a HashMap
    }

    // Collect the entries into a vector and sort by key
    let mut sorted_entries: Vec<_> = obj.iter().collect();
    sorted_entries.sort_by(|a, b| a.0.cmp(b.0));

    // Create the output string
    let mut output = String::new();
    output.push('{');

    for (i, (city, temps)) in sorted_entries.iter().enumerate() {
        let min_value = temps.iter().cloned().reduce(f64::min).unwrap();
        let max_value = temps.iter().cloned().reduce(f64::max).unwrap();
        let sum: f64 = temps.iter().sum();
        let avg_value = sum / temps.len() as f64;

        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&format!("{}={:.1}/{:.1}/{:.1}", city, min_value, avg_value, max_value));
    }

    output.push('}');
    // println!("{}", output);

    // Write the output to a file
    let mut file = File::create("../data/output.txt")?;
    file.write_all(output.as_bytes())?;
 
    // println!("{}", output);

    // Print the final HashMap to verify
    // for (city, temps) in &obj {
    //     let minValue = temps.iter().cloned().reduce(f64::min);
    //     let maxValue = temps.iter().cloned().reduce(f64::max);
    //     let sum: f64 = temps.iter().sum::<f64>();
    //     let avgValue = sum / temps.len() as f64;
    //     println!("City: {}, Temperatures: {:?},  Number of records: {:?}, Min: {:?}, Max: {:?}, Average: {:?}",
    //     city, 
    //     temps, 
    //     temps.len(),
    //     minValue,
    //     maxValue,
    //     avgValue);
    // }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "../data/weather_stations.csv";
    read_csv(filename)
}
