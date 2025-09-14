/// Main error type for QR payment operations
#[derive(Debug, thiserror::Error)]
pub enum QRError {
    #[error("Invalid QR format: {message}")]
    InvalidFormat { message: String },

    #[error("CRC validation failed")]
    InvalidCRC,

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid field value for {field}: {value}")]
    InvalidValue { field: String, value: String },

    #[error("Field value too long for {field}: {length} > {max_length}")]
    ValueTooLong {
        field: String,
        length: usize,
        max_length: usize,
    },

    #[error("Unsupported scheme: {scheme}")]
    UnsupportedScheme { scheme: String },

    #[error("QR payload too long: {length} > 512")]
    PayloadTooLong { length: usize },

    #[error("Builder error: {message}")]
    BuilderError { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

pub type Result<T> = std::result::Result<T, QRError>;
