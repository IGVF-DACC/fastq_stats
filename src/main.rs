use std::env::args;
use fastq::parse_path;

fn main() {
    let filename = args().nth(1);
    let path = match filename.as_ref().map(String::as_ref) {
        None|Some("-") => { None },
        Some(name) => Some(name)
    };

    let mut total: usize = 0;
    let mut sum: usize = 0;
    let mut current_read_length: usize = 0;
    let mut maximum_read_length: usize = 0;
    let mut minimum_read_length: usize = usize::MAX;
    parse_path(path, |parser| {
        parser.each(|record| {
            total += 1;
            current_read_length = record.to_owned_record().seq.len();
            sum += current_read_length;
            if current_read_length > maximum_read_length {
                maximum_read_length = current_read_length;
            }
            if current_read_length < minimum_read_length {
                minimum_read_length = current_read_length;
            }
            // return `true` if you want to continue iterating
            true
        }).expect("Invalid fastq file");
    }).expect("Invalid Compression");
        let mean: f64 = if total > 0 {
            (sum as f64) / (total as f64)
        } else {
            0.0
        };

    println!("read_count: {}", total);
    println!("minimum_read_length: {}", minimum_read_length);
    println!("maximum_read_length: {}", maximum_read_length);
    println!("mean_read_length: {}", mean);
}
