use std::{
  collections::{BTreeMap, BTreeSet, HashSet},
  ops::AddAssign,
  usize,
};

use esx_lib::{esx::ESx, record::Record, signature::Signature};

pub mod fingerprinting;

#[derive(Debug, Default)]
pub struct RecordTypeLayout {
  total_count: usize,
  fields: BTreeMap<Signature, Vec<usize>>,
}
impl RecordTypeLayout {
  pub fn total_count(&self) -> &usize {
    &self.total_count
  }
  pub fn fields(&self) -> &BTreeMap<Signature, Vec<usize>> {
    &self.fields
  }
}
impl RecordTypeLayout {
  pub fn new() -> Self {
    RecordTypeLayout::default()
  }
  pub fn process_record(&mut self, record: &Record) {
    self.total_count += 1;
    let mut record_fields: BTreeMap<Signature, usize> = BTreeMap::new();
    for field in record.get_data().get_fields() {
      let field_sig = *field.get_signature();
      record_fields.entry(field_sig).or_insert(0).add_assign(1);
    }
    for (sig, count) in record_fields {
      self.fields.entry(sig).or_default().push(count);
    }
  }

  pub fn is_always_present(&self, sig: &Signature) -> bool {
    let Some(field) = self.fields.get(sig) else {
      return false;
    };
    field.len() == self.total_count
  }
  pub fn field_mean(&self, sig: &Signature) -> f64 {
    let Some(field) = self.fields.get(sig) else {
      return 0.0;
    };
    let sum: usize = field.iter().sum();
    sum as f64 / self.total_count as f64
  }
  pub fn field_min(&self, sig: &Signature) -> usize {
    let Some(field) = self.fields.get(sig) else {
      return 0;
    };
    if field.len() != self.total_count {
      return 0;
    }
    *field.iter().min().unwrap()
  }
  pub fn field_max(&self, field: &Signature) -> usize {
    let Some(field) = self.fields.get(field) else {
      return 0;
    };
    *field.iter().max().unwrap()
  }
}

pub fn get_signatures(records: &Vec<&Record>) -> BTreeSet<Signature> {
  let mut signatures: HashSet<&Signature> = HashSet::new();
  for record in records {
    signatures.insert(record.get_signature());
  }
  let signatures: BTreeSet<Signature> = signatures.iter().map(|x| **x).collect();
  signatures
}
pub fn get_form_versions(records: &Vec<&Record>) -> BTreeSet<u16> {
  let mut form_versions: BTreeSet<u16> = BTreeSet::new();
  for record in records {
    form_versions.insert(*record.get_form_version());
  }
  form_versions
}

#[derive(Debug)]
pub struct StatReport {
  header: Record,
  record_count: usize,
  signatures: BTreeSet<Signature>,
  form_versions: BTreeSet<u16>,
  record_data_layouts: BTreeMap<Signature, BTreeMap<u16, RecordTypeLayout>>,
}
impl StatReport {
  pub fn header(&self) -> &Record {
    &self.header
  }
  pub fn record_count(&self) -> &usize {
    &self.record_count
  }
  pub fn signatures(&self) -> &BTreeSet<Signature> {
    &self.signatures
  }
  pub fn form_versions(&self) -> &BTreeSet<u16> {
    &self.form_versions
  }
  pub fn record_data_layouts(&self) -> &BTreeMap<Signature, BTreeMap<u16, RecordTypeLayout>> {
    &self.record_data_layouts
  }
}
impl StatReport {
  pub fn new(mut esx: ESx) -> Self {
    let mut report: StatReport = StatReport {
      header: esx.get_header_record().clone(),
      record_count: 0,
      signatures: BTreeSet::new(),
      form_versions: BTreeSet::new(),
      record_data_layouts: BTreeMap::new(),
    };
    esx.process();

    let records = esx.get_all_records();
    report.process_records(records);

    report
  }
  fn process_records(&mut self, records: Vec<&Record>) {
    self.record_count = records.len();
    self.signatures = get_signatures(&records);
    self.form_versions = get_form_versions(&records);

    for record in records {
      let sig = *record.get_signature();
      let form_version = *record.get_form_version();

      let layout = self
        .record_data_layouts
        .entry(sig)
        .or_default()
        .entry(form_version)
        .or_default();
      layout.process_record(record)
    }
  }
}
