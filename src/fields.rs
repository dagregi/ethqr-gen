use crate::EMVTag;
use crate::error::{QRError, Result};
use crate::tags;

/// Additional data fields (tag 62)
#[derive(Debug, Default, Clone)]
pub struct AdditionalData {
    /// Bill/Invoice/Voucher number (tag 01)
    pub bill_number: Option<String>,
    /// Mobile number of the merchant (tag 02)
    pub mobile_number: Option<String>,
    /// Branch name of the merchant (tag 03)
    pub store_label: Option<String>,
    /// Loyalty program identifier (tag 04)
    pub loyalty_number: Option<String>,
    /// Reference number for transaction (tag 05)
    pub reference_label: Option<String>,
    /// Customer identifier (tag 06)
    pub customer_label: Option<String>,
    /// Terminal/counter ID (tag 07)
    pub terminal_number: Option<String>,
    /// Purpose of transaction (tag 08)
    pub purpose: Option<String>,
    /// Additional customer data request (tag 09)
    pub additional_customer_data: Option<String>,
    /// Merchant tax ID (tag 10)
    pub merchant_tax_id: Option<String>,
    /// Merchant channel characteristics (tag 11)
    pub merchant_channel: Option<String>,
    /// Due date (DDMMYYYY format) (tag 50)
    pub due_date: Option<String>,
    /// Amount after due date (tag 51)
    pub amount_after_due_date: Option<String>,
}

impl AdditionalData {
    #[must_use]
    pub fn new() -> AdditionalData {
        AdditionalData::default()
    }

    pub fn bill_number(mut self, bill_number: impl Into<String>) -> Self {
        self.bill_number = Some(bill_number.into());
        self
    }

    pub fn mobile_number(mut self, mobile_number: impl Into<String>) -> Self {
        self.mobile_number = Some(mobile_number.into());
        self
    }

    pub fn store_label(mut self, store_label: impl Into<String>) -> Self {
        self.store_label = Some(store_label.into());
        self
    }

    pub fn loyalty_number(mut self, loyalty_number: impl Into<String>) -> Self {
        self.loyalty_number = Some(loyalty_number.into());
        self
    }

    pub fn reference_label(mut self, reference_label: impl Into<String>) -> Self {
        self.reference_label = Some(reference_label.into());
        self
    }

    pub fn customer_label(mut self, customer_label: impl Into<String>) -> Self {
        self.customer_label = Some(customer_label.into());
        self
    }

    pub fn terminal_number(mut self, terminal_number: impl Into<String>) -> Self {
        self.terminal_number = Some(terminal_number.into());
        self
    }

    pub fn purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    pub fn additional_customer_data(mut self, additional_customer_data: impl Into<String>) -> Self {
        self.additional_customer_data = Some(additional_customer_data.into());
        self
    }

    pub fn merchant_tax_id(mut self, merchant_tax_id: impl Into<String>) -> Self {
        self.merchant_tax_id = Some(merchant_tax_id.into());
        self
    }

    pub fn merchant_channel(mut self, merchant_channel: impl Into<String>) -> Self {
        self.merchant_channel = Some(merchant_channel.into());
        self
    }

    /// Due date (DDMMYYYY format)
    pub fn due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }

    pub fn amount_after_due_date(mut self, amount_after_due_date: impl Into<String>) -> Self {
        self.amount_after_due_date = Some(amount_after_due_date.into());
        self
    }

    /// Encode additional data as EMV tag
    #[must_use]
    pub fn encode(&self) -> Option<EMVTag> {
        let mut sub_tags = Vec::new();

        if let Some(ref value) = self.bill_number {
            sub_tags.push(EMVTag::new("01", value));
        }
        if let Some(ref value) = self.mobile_number {
            sub_tags.push(EMVTag::new("02", value));
        }
        if let Some(ref value) = self.store_label {
            sub_tags.push(EMVTag::new("03", value));
        }
        if let Some(ref value) = self.loyalty_number {
            sub_tags.push(EMVTag::new("04", value));
        }
        if let Some(ref value) = self.reference_label {
            sub_tags.push(EMVTag::new("05", value));
        }
        if let Some(ref value) = self.customer_label {
            sub_tags.push(EMVTag::new("06", value));
        }
        if let Some(ref value) = self.terminal_number {
            sub_tags.push(EMVTag::new("07", value));
        }
        if let Some(ref value) = self.purpose {
            sub_tags.push(EMVTag::new("08", value));
        }
        if let Some(ref value) = self.additional_customer_data {
            sub_tags.push(EMVTag::new("09", value));
        }
        if let Some(ref value) = self.merchant_tax_id {
            sub_tags.push(EMVTag::new("10", value));
        }
        if let Some(ref value) = self.merchant_channel {
            sub_tags.push(EMVTag::new("11", value));
        }
        if let Some(ref value) = self.due_date {
            sub_tags.push(EMVTag::new("50", value));
        }
        if let Some(ref value) = self.amount_after_due_date {
            sub_tags.push(EMVTag::new("51", value));
        }

        if sub_tags.is_empty() {
            None
        } else {
            let value = sub_tags
                .iter()
                .map(super::EMVTag::encode)
                .collect::<String>();
            Some(EMVTag::new(tags::ADDITIONAL_DATA, value))
        }
    }
}

/// Extension fields for tags 80-99
#[derive(Debug, Default, Clone)]
pub struct ExtensionFields {
    /// Context/particulars of transaction (tag 80)
    pub transaction_context: Option<String>,
    /// Discounts & loyalty programs (tag 81)
    pub discounts_loyalty: Option<String>,
    /// Offline to online payments (tag 82)
    pub offline_to_online: Option<String>,
    /// E-commerce related data (tag 83)
    pub ecommerce: Option<String>,
    /// End-to-end ID for dynamic QR with RTP (tag 84)
    pub end_to_end_id: Option<String>,
    /// Transaction Type Code (tag 85)
    pub transaction_type_code: Option<String>,
}

/// Convenience fee configuration
#[derive(Debug, Clone)]
pub enum ConvenienceFee {
    /// Prompt customer to add tip
    Prompt,
    /// Fixed tip amount
    Fixed(String),
    /// Percentage-based tip
    Percentage(String),
}

/// Payment scheme configuration
#[derive(Debug, Clone)]
pub enum SchemeConfig {
    Visa {
        account_info: String,
    },
    Mastercard {
        account_info: String,
    },
    UnionPay {
        account_info: String,
    },
    IPSET {
        guid: String,
        bic: String,
        account: String,
    },
}

impl SchemeConfig {
    /// Create IPS ET scheme builder
    #[must_use]
    pub fn ips_et(guid: &str, bic: &str, account: &str) -> Self {
        SchemeConfig::IPSET {
            guid: guid.to_string(),
            bic: bic.to_string(),
            account: account.to_string(),
        }
    }

    /// Create Visa scheme
    pub fn visa(account_info: impl Into<String>) -> Self {
        Self::Visa {
            account_info: account_info.into(),
        }
    }

    /// Create Mastercard scheme
    pub fn mastercard(account_info: impl Into<String>) -> Self {
        Self::Mastercard {
            account_info: account_info.into(),
        }
    }

    /// Get the scheme tag ID
    #[must_use]
    pub fn tag_id(&self) -> &str {
        match self {
            SchemeConfig::Visa { .. } => tags::VISA,
            SchemeConfig::Mastercard { .. } => tags::MASTERCARD,
            SchemeConfig::UnionPay { .. } => tags::UNIONPAY,
            SchemeConfig::IPSET { .. } => tags::IPS_ET,
        }
    }

    /// Encode scheme as EMV tag
    pub fn encode(&self) -> Result<EMVTag> {
        match self {
            SchemeConfig::Visa { account_info } => Ok(EMVTag::new(tags::VISA, account_info)),
            SchemeConfig::Mastercard { account_info } => {
                Ok(EMVTag::new(tags::MASTERCARD, account_info))
            }
            SchemeConfig::UnionPay { account_info } => {
                Ok(EMVTag::new(tags::UNIONPAY, account_info))
            }
            SchemeConfig::IPSET { guid, bic, account } => {
                // Validate GUID format (UUID without hyphens)
                if guid.len() != 32 || !guid.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(QRError::InvalidValue {
                        field: "guid".to_string(),
                        value: guid.clone(),
                    });
                }

                // Validate BIC format (8 or 11 characters)
                if bic.len() != 8 && bic.len() != 11 {
                    return Err(QRError::InvalidValue {
                        field: "bic".to_string(),
                        value: bic.clone(),
                    });
                }

                // Validate account format
                if account.len() > 24 {
                    return Err(QRError::InvalidValue {
                        field: "account".to_string(),
                        value: account.clone(),
                    });
                }

                // Build sub-tags
                let sub_tag_00 = EMVTag::new("00", guid);
                let sub_tag_01 = EMVTag::new("01", bic);
                let sub_tag_02 = EMVTag::new("02", account);

                let value = format!(
                    "{}{}{}",
                    sub_tag_00.encode(),
                    sub_tag_01.encode(),
                    sub_tag_02.encode()
                );

                Ok(EMVTag::new(tags::IPS_ET, value))
            }
        }
    }
}
