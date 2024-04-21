// Provide a statistical analysis of the records and fields in the ESx
// Record reports should be grouped by both signature and form version
// Record reports should include the mimimum, maximum, and average number of each field type, as well as the total number of records used for the average
// Field reports should be grouped by signature, their containing record type, and associated form version
// Field reports should include the minimum, maximum, and average length of their data
// Full report should be exported as RON

pub fn main() {
  let args: Vec<String> = std::env::args().collect();
  let esx = esx_bin::load::from_file(&args[1]).unwrap();
  let stats = esx_bin::statistics::StatReport::new(esx);

  println!("Signatures: {:?}", stats.signatures());
  println!("Form Versions: {:?}", stats.form_versions());
  for (sig, versions) in stats.record_data_layouts() {
    println!("Signature: {}", sig);
    for (version, layout) in versions {
      println!("\tVersion: {}", version);
      println!("\t\tTotal: {}", layout.total_count());
      for field in layout.fields().keys() {
        println!(
          "\t\tField: {} ( |{}|{}|{}| )",
          field,
          layout.field_min(field),
          layout.field_mean(field),
          layout.field_max(field)
        );
      }
    }
  }
}
