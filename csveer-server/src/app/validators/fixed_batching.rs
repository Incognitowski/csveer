use crate::config::server::AppError;

pub fn validate(batch_size: &i32) -> anyhow::Result<(), AppError> {
    if batch_size.is_negative() || batch_size == &0 {
        return Err(AppError::DetailedValidation(
            String::from("Invalid fixed batch configuration"),
            vec![format!(
                "Batch size should be a positive number. Provided: {}",
                batch_size
            )],
        ));
    }

    Ok(())
}
