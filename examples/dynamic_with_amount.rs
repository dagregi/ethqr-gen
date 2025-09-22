//! Basic dynamic QR code example
//!
//! This example demonstrates creating a simple dynamic QR code

use ethqr_gen::{
    QRBuilder,
    fields::{AdditionalData, SchemeConfig},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Additional data if required
    let additional_data = AdditionalData::new()
        .bill_number("INV-001")
        .reference_label("ORDER-123");

    // Create a basic dynamic QR code for a restaurant
    let qr_builder = QRBuilder::new()
        .merchant_name("Restaurant")
        .merchant_city("Dire Dawa")
        .merchant_category_code("5812")
        .add_scheme(SchemeConfig::ips_et(
            "581b314e257f41bfbbdc6384daa31d16",
            "CBETETAA",
            "10000171234567890",
        ))
        .transaction_amount("50.00")
        .additional_data(additional_data);

    let qr_code = qr_builder.build()?;

    println!("QR Code Payload: {}", qr_builder);
    println!("QR Code version: {:?}", qr_code.version());

    Ok(())
}
