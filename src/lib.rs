//! # ethqr-gen
//!
//! A Rust library for generating EMVCo-compliant QR codes for payments according
//! to the Ethiopian Interoperable QR Standard.
//!
//! ## Features
//!
//! - EMVCo QR Code standard compliance
//! - Support for multiple payment schemes (Visa, Mastercard, IPS ET, etc.)
//! - Static and dynamic QR code generation
//!
//! ## Quick Start
//!
//! ### Static QR Code (no predefined amount)
//!
//! ```rust
//! use ethqr_gen::{QRBuilder, fields::SchemeConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let qr_code = QRBuilder::new()
//!     .merchant_name("Coffee Shop")
//!     .merchant_city("Addis Ababa")
//!     .merchant_category_code("5812") // Restaurant
//!     .add_scheme(SchemeConfig::visa("4111111111111111"))
//!     .build()?;
//!
//! println!("Static QR: {}", qr_code);
//! # Ok(())
//! # }
//! ```
//!
//! ### Dynamic QR Code (with specific amount)
//!
//! ```rust
//! use ethqr_gen::{QRBuilder, fields::{SchemeConfig, AdditionalData}};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let additional_data = AdditionalData::new()
//!     .bill_number("INV-001")
//!     .reference_label("ORDER-123");
//!
//! let qr_code = QRBuilder::new()
//!     .merchant_name("Restaurant")
//!     .merchant_city("Dire Dawa")
//!     .merchant_category_code("5812")
//!     .add_scheme(SchemeConfig::ips_et(
//!         "581b314e257f41bfbbdc6384daa31d16",
//!         "CBETETAA",
//!         "10000171234567890",
//!     ))
//!     .transaction_amount("50.00")
//!     .additional_data(additional_data)
//!     .build()?;
//!
//! println!("Dynamic QR: {}", qr_code);
//! # Ok(())
//! # }
//! ```
//!
//! ## Payment Schemes
//!
//! The library supports multiple payment schemes through the [`SchemeConfig`](fields::SchemeConfig) type:
//!
//! - **Visa**: `SchemeConfig::visa("account_info")`
//! - **Mastercard**: `SchemeConfig::mastercard("account_info")`
//! - **UnionPay**: `SchemeConfig::unionpay("account_info")`
//! - **IPS ET**: `SchemeConfig::ips_et("guid", "bic", "account_info")` (Ethiopian Interbank Payment System)

pub mod crc;
pub mod error;
pub mod fields;

use crate::error::{QRError, Result};
use crate::fields::{AdditionalData, SchemeConfig};

pub mod constants {
    pub const PAYLOAD_FORMAT_INDICATOR: &str = "01";
    pub const ETB_CURRENCY_CODE: &str = "230";
    pub const ETHIOPIA_COUNTRY_CODE: &str = "ET";
    pub const MAX_QR_LENGTH: usize = 512;
    pub const STATIC_QR_POI: &str = "11";
    pub const DYNAMIC_QR_POI: &str = "12";
    pub const MAX_MERCHANT_NAME_LEN: usize = 25;
    pub const MAX_MERCHANT_CITY_LEN: usize = 15;
}

pub mod tags {
    pub const PAYLOAD_FORMAT_INDICATOR: &str = "00";
    pub const POINT_OF_INITIATION: &str = "01";
    pub const MERCHANT_CATEGORY_CODE: &str = "52";
    pub const TRANSACTION_CURRENCY: &str = "53";
    pub const TRANSACTION_AMOUNT: &str = "54";
    pub const COUNTRY_CODE: &str = "58";
    pub const MERCHANT_NAME: &str = "59";
    pub const MERCHANT_CITY: &str = "60";
    pub const ADDITIONAL_DATA: &str = "62";
    pub const CRC: &str = "63";
    pub const ALTERNATE_LANGUAGE: &str = "64";
    pub const TRANSACTION_CONTEXT: &str = "80";

    // Scheme allocations
    pub const VISA: &str = "02";
    pub const MASTERCARD: &str = "04";
    pub const UNIONPAY: &str = "15";
    pub const IPS_ET: &str = "28";
}

/// Represents an EMV tag with ID, length, and value
#[derive(Debug, Clone, PartialEq)]
pub struct EMVTag {
    pub id: String,
    pub value: String,
}

impl EMVTag {
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }

    /// Get the length of the value
    pub fn length(&self) -> usize {
        self.value.len()
    }

    /// Encode as TLV string
    pub fn encode(&self) -> String {
        format!("{}{:02}{}", self.id, self.length(), self.value)
    }
}

/// Builder for constructing QR codes
#[derive(Default, Clone)]
pub struct QRBuilder {
    payload_format_indicator: String,
    point_of_initiation: Option<String>,
    merchant_name: String,
    merchant_city: String,
    merchant_category_code: String,
    schemes: Vec<SchemeConfig>,
    transaction_amount: Option<String>,
    transaction_currency: String,
    additional_data: Option<AdditionalData>,
    transaction_context: Option<String>,
}

impl QRBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set merchant name
    pub fn merchant_name(mut self, name: impl Into<String>) -> Self {
        self.merchant_name = name.into();
        self
    }

    /// Set merchant city
    pub fn merchant_city(mut self, city: impl Into<String>) -> Self {
        self.merchant_city = city.into();
        self
    }

    /// Set merchant category code (ISO 4217)
    pub fn merchant_category_code(mut self, code: impl Into<String>) -> Self {
        self.merchant_category_code = code.into();
        self
    }

    /// Add a payment scheme
    pub fn add_scheme(mut self, scheme: SchemeConfig) -> Self {
        self.schemes.push(scheme);
        self
    }

    /// Set transaction amount (for dynamic QR)
    pub fn transaction_amount(mut self, amount: impl Into<String>) -> Self {
        self.transaction_amount = Some(amount.into());
        self
    }

    /// Set additional data
    pub fn additional_data(mut self, data: AdditionalData) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set transaction context
    pub fn transaction_context(mut self, context: impl Into<String>) -> Self {
        self.transaction_context = Some(context.into());
        self
    }

    fn validate(&self) -> Result<()> {
        // Validate merchant information
        if self.merchant_name.len() > constants::MAX_MERCHANT_NAME_LEN {
            return Err(QRError::ValueTooLong {
                field: "name".to_string(),
                length: self.merchant_name.len(),
                max_length: constants::MAX_MERCHANT_NAME_LEN,
            });
        }

        if self.merchant_city.len() > constants::MAX_MERCHANT_CITY_LEN {
            return Err(QRError::ValueTooLong {
                field: "city".to_string(),
                length: self.merchant_city.len(),
                max_length: constants::MAX_MERCHANT_CITY_LEN,
            });
        }

        // Validate category code format (4 digits)
        if self.merchant_category_code.len() != 4
            || !self
                .merchant_category_code
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            return Err(QRError::InvalidValue {
                field: "category_code".to_string(),
                value: self.merchant_category_code.clone(),
            });
        }

        if self.schemes.is_empty() {
            return Err(QRError::MissingField {
                field: "schemes".to_string(),
            });
        }

        Ok(())
    }

    /// Build the QR code
    pub fn build(&mut self) -> Result<String> {
        self.validate()?;

        self.point_of_initiation = if self.transaction_amount.is_some() {
            Some(constants::DYNAMIC_QR_POI.to_string())
        } else {
            Some(constants::STATIC_QR_POI.to_string())
        };

        self.payload_format_indicator = constants::PAYLOAD_FORMAT_INDICATOR.to_string();
        self.transaction_currency = constants::ETB_CURRENCY_CODE.to_string();

        let mut tags = Vec::new();

        // Payload Format Indicator (mandatory)
        tags.push(EMVTag::new(
            tags::PAYLOAD_FORMAT_INDICATOR,
            &self.payload_format_indicator,
        ));

        // Point of Initiation (optional)
        if let Some(ref poi) = self.point_of_initiation {
            tags.push(EMVTag::new(tags::POINT_OF_INITIATION, poi));
        }

        // Merchant Account Information (schemes)
        for scheme in &self.schemes {
            tags.push(scheme.encode()?);
        }

        // Merchant Category Code (mandatory)
        tags.push(EMVTag::new(
            tags::MERCHANT_CATEGORY_CODE,
            &self.merchant_category_code,
        ));

        // Transaction Currency (mandatory)
        tags.push(EMVTag::new(
            tags::TRANSACTION_CURRENCY,
            &self.transaction_currency,
        ));

        // Transaction Amount (optional)
        if let Some(ref amount) = self.transaction_amount {
            tags.push(EMVTag::new(tags::TRANSACTION_AMOUNT, amount));
        }

        // Country Code (mandatory)
        tags.push(EMVTag::new(
            tags::COUNTRY_CODE,
            constants::ETHIOPIA_COUNTRY_CODE,
        ));

        // Merchant Name (mandatory)
        tags.push(EMVTag::new(tags::MERCHANT_NAME, &self.merchant_name));

        // Merchant City (mandatory)
        tags.push(EMVTag::new(tags::MERCHANT_CITY, &self.merchant_city));

        // Additional Data (optional)
        if let Some(ref additional_data) = self.additional_data
            && let Some(tag) = additional_data.encode()
        {
            tags.push(tag);
        }

        // Transaction Context (optional)
        if let Some(ref context) = self.transaction_context {
            tags.push(EMVTag::new(tags::TRANSACTION_CONTEXT, context));
        }

        // Build payload without CRC
        let mut payload = tags.iter().map(|tag| tag.encode()).collect::<String>();

        // Calculate and append CRC
        let crc = crc::calculate_crc16(&format!("{}6304", payload));
        payload.push_str(&format!("63{:02}{}", crc.len(), crc));

        // Validate length
        if payload.len() > constants::MAX_QR_LENGTH {
            return Err(QRError::PayloadTooLong {
                length: payload.len(),
            });
        }

        Ok(payload)
    }
}
