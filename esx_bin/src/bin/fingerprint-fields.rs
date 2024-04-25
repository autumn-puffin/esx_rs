// Fingerprints the fields of a provided ESx file

use std::collections::BTreeMap;

use esx_bin::{
  load,
  statistics::fingerprinting::{FieldFingerprint, FieldFingerprinting},
};
use esx_lib::{field::Field, types::Signature};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let mut esx = match load::from_file(&args[1]) {
    Ok(esx) => esx,
    Err(e) => {
      println!("Error loading file: {:?}", e);
      return;
    }
  };
  esx.process();

  let all_records = esx.get_all_records();
  let mut fields_by_signature_path: BTreeMap<(Signature, u16, Signature), Vec<&Field>> =
    BTreeMap::new();
  for record in all_records {
    let rec_sig = *record.get_signature();
    let rec_ver = *record.get_form_version();
    for field in record.get_data().get_fields() {
      let field_sig = *field.get_signature();
      let field_path = (rec_sig, rec_ver, field_sig);
      fields_by_signature_path
        .entry(field_path)
        .or_insert_with(Vec::new)
        .push(field);
    }
  }

  let mut fingerprints: BTreeMap<String, FieldFingerprint> = BTreeMap::new();

  for (path, fields) in fields_by_signature_path {
    let path_str = path.0.as_string() + ":" + &path.1.to_string() + " -> " + &path.2.as_string();
    let fingerprint = fields.fingerprint();
    fingerprints.insert(path_str, fingerprint);
  }
  println!(
    "{}",
    ron::ser::to_string_pretty(&fingerprints, Default::default()).unwrap()
  )
}
