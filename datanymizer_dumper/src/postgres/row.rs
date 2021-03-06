use crate::{ColumnData, Table};
use anyhow::Result;
use datanymizer_engine::{Engine, StringValue};
use postgres::types::Type;
use std::char;

#[derive(Debug)]
pub struct PgRow<T>
where
    T: Table<Type>,
{
    table: T,
    source: String,
}

impl<T> PgRow<T>
where
    T: Table<Type>,
{
    pub fn from_string_row(source: String, parent_table: T) -> Self {
        Self {
            source,
            table: parent_table,
        }
    }

    /// Apply transform engine to every colomn in row
    /// Returs new StringRecord for store in dump
    pub fn transform(&self, engine: &Engine) -> Result<String> {
        let mut result: Vec<String> = Vec::new();
        let split_char: char = char::from_u32(0x0009).unwrap();
        let cols: Vec<&str> = self.source.split(split_char).collect();
        for (ind, col) in self.table.get_columns().iter().enumerate() {
            let pos = ind;
            let value = cols.get(pos).unwrap_or(&"");
            let mut dto = StringValue {
                table_name: self.table.get_name().clone(),
                field_name: col.name().to_string(),
                value: value.to_string(),
            };
            let _ = engine.process(&mut dto)?;

            dto.update(dto.value.clone());

            result.insert(pos, dto.value);
        }
        Ok(result.join("\t"))
    }
}
