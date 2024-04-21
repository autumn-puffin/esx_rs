// Dump the ESX binary to a RON file
fn main() {
  // load a file from args
  // process the groups and reocrds
  let args: Vec<String> = std::env::args().collect();
  println!("{:?}", args);
  let mut esx = esx_bin::load::from_file(&args[1]).unwrap();
  esx.process();

  esx_bin::save::to_ron_file(&esx, &args[2]).unwrap();
}
