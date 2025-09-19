use anyhow::Context;
use url::Url;
use ai_core::domain::models::{Height, Kilogram, Metre, Weight};
use std::fmt::Write;

pub mod postgres_ai_entity_repository;
mod postgres_gender_repository;
mod postgres_personality_repository;

pub trait Row<T> {
    fn from_row(value: T) -> Result<Self, anyhow::Error>
    where Self: Sized;
    fn from_rows(values: Vec<T>) -> Result<Vec<Self>, anyhow::Error>
    where Self: Sized { values.into_iter().map(Self::from_row).collect() }
    fn to_row(&self) -> T;
}

impl Row<f32> for Height {
    fn from_row(value: f32) -> Result<Self, anyhow::Error> {
        Self::new(Metre::new(value))
            .with_context(|| format!("Failed to parse Height from database value: '{}'", value))
    }
    fn to_row(&self) -> f32 { *self.as_metre().as_f32() }
}

impl Row<f32> for Weight {
    fn from_row(value: f32) -> Result<Self, anyhow::Error> {
        Self::new(Kilogram::new(value))
            .with_context(|| format!("Failed to parse Weight from database value: '{}'", value))
    }
    fn to_row(&self) -> f32 { *self.as_kilogram().as_f32() }
}

impl Row<String> for Url {
    fn from_row(value: String) -> Result<Self, anyhow::Error> { Ok(Self::parse(&value)?) }
    fn to_row(&self) -> String { self.to_string() }
}

fn build_insert_query(table: &str, num_rows: usize, columns: &[&str]) -> String {
    let cols = columns.join(", ");
    let ncols = columns.len();

    let mut sql = String::with_capacity(32 + cols.len() + num_rows * (4 * ncols + 3) + 16);

    write!(&mut sql, "INSERT INTO {} ({}) VALUES ", table, cols).unwrap();

    for r in 0..num_rows {
        if r > 0 { sql.push(','); }
        sql.push('(');
        for c in 0..ncols {
            if c > 0 { sql.push(','); }
            let idx = r * ncols + c + 1; // Postgres is 1-based
            write!(&mut sql, "${}", idx).unwrap();
        }
        sql.push(')');
    }

    sql.push_str(" RETURNING *");
    sql
    // let placeholders: Vec<String> = (0..num_rows)
    //     .map(|i| {
    //         let start = i*columns.len() + 1;
    //         let end = (i + 1)*columns.len() + 1;
    //         let row_placeholders: Vec<String> = (start..end)
    //             .map(|j| {
    //                 format!("${}", j)
    //             })
    //             .collect();
    //         format!("({})", row_placeholders.join(", "))
    //     })
    //     .collect();
    //
    // format!(
    //     "INSERT INTO {} ({}) VALUES {} RETURNING *",
    //     table,
    //     columns.join(", "),
    //     placeholders.join(", "),
    // )
}