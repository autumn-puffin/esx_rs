use esx_bin::load;
use esx_lib::group::GroupLabel;
use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let mut esx = load::from_file(&args[1]).unwrap();
  esx.process();

  println!("Header: {:#?}", esx.get_header_record());
  for group in esx.get_top_groups() {
    if let GroupLabel::Top(s) = group.get_label() {
      println!(
        "{} - Len: {:0x?}",
        s.as_string(),
        group.get_data().as_bytes().len(),
      )
    }
  }
}
