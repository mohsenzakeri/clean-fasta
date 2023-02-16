use std::env;
use std::io;

use bio::io::fasta;

// https://stackoverflow.com/questions/67082301/how-to-read-in-a-fasta-in-rust-usio-the-bio-package

fn main() {
  let src = env::args().nth(1).expect("missing src");
  let reader = fasta::Reader::from_file(src).unwrap();
  let mut writer = fasta::Writer::new(io::stdout());
  let mut counter = 0;
  for result in reader.records() {
    if counter % 100 == 0 {
        eprint!("{}\r", counter);
    }
    counter += 1;
    let result_data = &result.unwrap();

    // Got: 65 A
    // Got: 67 C
    // Got: 71 G
    // Got: 84 T
    let mut new_seq = Vec::<u8>::new();
    for &val in result_data.seq() {
      if val != 65 && val != 67 && val != 71 && val != 84 {
        new_seq.push(65);
      } else {
        new_seq.push(val);
      }
      // println!("Got: {} {}", val, val as char);
    }
    // println!("{:?}", new_seq);
    
    let description = Some("");
    let new_record = fasta::Record::with_attrs(result_data.id(), description, &new_seq);
    writer.write_record(&new_record).ok().expect("Error writing record.");
    // let s_ = std::str::from_vec(new_seq).expect("invalid utf-8 sequence");
    // println!("{}", s_);
  }
  eprintln!("");
}
