use std::collections::HashSet;

use crate::config::server::AppError;

pub fn validate(columns: &[i32]) -> anyhow::Result<(), AppError> {
    if columns.is_empty() {
        return Err(AppError::DetailedValidation(
            String::from("Column grouping configuration is invalid"),
            vec![String::from("No column index provided")],
        ));
    }

    let mut col_set: HashSet<&i32> = HashSet::new();

    for (idx, col) in columns.iter().enumerate() {
        if col.is_negative() {
            return Err(AppError::DetailedValidation(
                String::from("Column grouping configuration is invalid"),
                vec![format!(
                    "Column index at position {} cannot be negative",
                    idx
                )],
            ));
        }
        if col_set.contains(col) {
            return Err(AppError::DetailedValidation(
                String::from("Column grouping configuration is invalid"),
                vec![format!(
                    "Column index {} appears twice in columns list",
                    col
                )],
            ));
        }
        col_set.insert(col);
    }

    Ok(())
}
