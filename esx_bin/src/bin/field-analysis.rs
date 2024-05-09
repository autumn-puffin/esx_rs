// Intended Usage: field-analysis --file <path> --record <record-sig> --field <field-sig> --form-version <form-version>
// Dump all fields of a specific type from a specific record type and optionally a specific form version

use esx_lib::{types::FormID, Field, Record};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let mut file = None;
  let mut record = None;
  let mut field = None;
  let mut form_version = None;

  // parse args
  let mut i = 1; // skip the first arg
  while i < args.len() {
    match args[i].as_str() {
      "--file" => {
        file = Some(args[i + 1].clone());
        i += 2;
      }
      "--record" => {
        record = Some(args[i + 1].clone());
        i += 2;
      }
      "--field" => {
        field = Some(args[i + 1].clone());
        i += 2;
      }
      "--form-version" => {
        form_version = Some(args[i + 1].clone());
        i += 2;
      }
      _ => {
        println!("Unrecognised argument: {}", args[i]);
        return;
      }
    }
  }

  let Some(file) = file else {
    println!("No file provided");
    return;
  };
  let Some(record) = record else {
    println!("No record provided");
    return;
  };
  let Some(field) = field else {
    println!("No field provided");
    return;
  };
  let mut esx = match esx_bin::load::from_file(&file) {
    Ok(esx) => esx,
    Err(e) => {
      println!("Error loading file: {:?}", e);
      return;
    }
  };
  esx.process();
  let mut records: Vec<&Record> = esx.get_all_records();
  records.retain(|x| x.get_signature().as_string() == record);

  if let Some(form_version) = form_version {
    let form_version = form_version.parse::<u16>().unwrap();
    records.retain(|x| *x.get_form_version() == form_version);
  }

  let mut fields: Vec<(&FormID, Vec<&Field>)> = vec![];
  for rec in records {
    let rec_fields = rec.get_data().get_fields();
    let rec_fields: Vec<&Field> = rec_fields
      .into_iter()
      .filter(|x| x.get_signature().as_string() == field)
      .collect();
    if !rec_fields.is_empty() {
      fields.push((rec.get_form_id(), rec_fields));
    }
  }
  println!(
    "Fields: {}",
    ron::ser::to_string_pretty(&fields, Default::default()).unwrap()
  );
}
