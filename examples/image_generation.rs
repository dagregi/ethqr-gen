//! QR Code Image Generation Example
//!
//! This example shows how to generate QR code images and save them as PNG files.
//! Requires the "qr-image" feature to be enabled.
//!
//! Run with: cargo run --features qr-image --example qr_image_generation

use ethqr_gen::{
    QRBuilder,
    fields::{AdditionalData, SchemeConfig},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a QR code for a taxi service
    let additional_data = AdditionalData::new()
        .reference_label("RIDE-789")
        .customer_label("Trip to Airport");

    let mut qr_builder = QRBuilder::new()
        .merchant_name("Addis Taxi")
        .merchant_city("Addis Ababa")
        .merchant_category_code("4121") // Taxi service
        .add_scheme(SchemeConfig::ips_et(
            "92ca314e257f41bfbbdc6384daa31d16",
            "CBETETAA",
            "10000187654321098",
        ))
        .transaction_amount("85.00")
        .additional_data(additional_data);

    // Generate QR code image with default size
    let qr_image = qr_builder.build_image()?;
    qr_image.save("/tmp/qr_image.png")?;
    println!("Default size QR image saved as: /tmp/qr_image.png");

    Ok(())
}
