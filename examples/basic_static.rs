//! Basic static QR code example
//!
//! This example demonstrates creating a simple static QR code

use ethqr_gen::{QRBuilder, fields::SchemeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a basic static QR code for a coffee shop
    let qr_builder = QRBuilder::new()
        .merchant_name("Addis Coffee House")
        .merchant_city("Addis Ababa")
        .merchant_category_code("5812") // Eating places, restaurants
        .add_scheme(SchemeConfig::visa("4111111111111111"))
        .add_scheme(SchemeConfig::ips_et(
            "581b314e257f41bfbbdc6384daa31d16",
            "CBETETAA",
            "10000171234567890",
        ));

    let qr_code = qr_builder.build()?;

    println!("QR Code Payload: {}", qr_builder);
    println!("QR Code version: {:?}", qr_code.version());

    Ok(())
}
