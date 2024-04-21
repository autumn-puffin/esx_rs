use super::Field;

pub enum StructuredField {
  Required(Field),
  Optional(Option<Field>),
}
