use std::{
  collections::{HashMap, VecDeque},
  env,
};

use esx_bin::{
  load,
  statistics::{get_signatures, RecordTypeLayout},
};
use esx_lib::{record::Record, signature::Signature};

// Dump record statistics of an ESX file
fn main() {
  // load a file from args
  // process the groups and reocrds
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let mut esx = load::from_file(&args[1]).unwrap();
  esx.process();

  let records = esx.get_all_records();
  let signatures = get_signatures(&records);

  // Collate the records by signature
  let mut record_map: HashMap<&Signature, VecDeque<&Record>> = HashMap::new();
  for sig in &signatures {
    record_map.insert(sig, VecDeque::new());
  }
  for record in &records {
    record_map
      .get_mut(&record.get_signature())
      .unwrap()
      .push_back(record);
  }

  // For each record type, get the number of each field type in each record
  let mut record_data_layouts: Vec<(&Signature, RecordTypeLayout)> = vec![];
  for (sig, records) in &record_map {
    let mut layout = RecordTypeLayout::new();
    for record in records {
      layout.process_record(record);
    }
    record_data_layouts.push((sig, layout));
  }

  // For each signature, generate a hashmap of the present form versions and their counts
  let mut form_version_counts: HashMap<&Signature, Vec<(u16, usize)>> = HashMap::new();
  for (sig, records) in &record_map {
    let mut form_version_count: HashMap<u16, usize> = HashMap::new();
    for record in records {
      let count = form_version_count
        .entry(*record.get_form_version())
        .or_insert(0);
      *count += 1;
    }
    let mut form_version_count: Vec<(u16, usize)> =
      form_version_count.iter().map(|(k, v)| (*k, *v)).collect();
    form_version_count.sort_by(|a, b| a.0.cmp(&b.0));
    form_version_counts.insert(sig, form_version_count);
  }

  // Print the statistics
  println!("Header {:#?}", esx.get_header_record());
  println!("Records: {:#?}", records.len());
  println!("Signatures {:#?}", signatures);

  println!("Form Version Counts {{");
  for (sig, counts) in &form_version_counts {
    println!("\t{}: {{", sig);
    for (form_version, count) in counts {
      println!("\t\t{}: {}", form_version, count);
    }
    println!("\t}}")
  }
  println!("}}");

  println!("Record Layouts {:#?}", record_data_layouts);
}
