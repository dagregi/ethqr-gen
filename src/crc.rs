// CRC-16-CCITT lookup table for fast calculation
const CRC16_CCITT_TABLE: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50a5, 0x60c6, 0x70e7, 0x8108, 0x9129, 0xa14a, 0xb16b,
    0xc18c, 0xd1ad, 0xe1ce, 0xf1ef, 0x1231, 0x0210, 0x3273, 0x2252, 0x52b5, 0x4294, 0x72f7, 0x62d6,
    0x9339, 0x8318, 0xb37b, 0xa35a, 0xd3bd, 0xc39c, 0xf3ff, 0xe3de, 0x2462, 0x3443, 0x0420, 0x1401,
    0x64e6, 0x74c7, 0x44a4, 0x5485, 0xa56a, 0xb54b, 0x8528, 0x9509, 0xe5ee, 0xf5cf, 0xc5ac, 0xd58d,
    0x3653, 0x2672, 0x1611, 0x0630, 0x76d7, 0x66f6, 0x5695, 0x46b4, 0xb75b, 0xa77a, 0x9719, 0x8738,
    0xf7df, 0xe7fe, 0xd79d, 0xc7bc, 0x48c4, 0x58e5, 0x6886, 0x78a7, 0x0840, 0x1861, 0x2802, 0x3823,
    0xc9cc, 0xd9ed, 0xe98e, 0xf9af, 0x8948, 0x9969, 0xa90a, 0xb92b, 0x5af5, 0x4ad4, 0x7ab7, 0x6a96,
    0x1a71, 0x0a50, 0x3a33, 0x2a12, 0xdbfd, 0xcbdc, 0xfbbf, 0xeb9e, 0x9b79, 0x8b58, 0xbb3b, 0xab1a,
    0x6ca6, 0x7c87, 0x4ce4, 0x5cc5, 0x2c22, 0x3c03, 0x0c60, 0x1c41, 0xedae, 0xfd8f, 0xcdec, 0xddcd,
    0xad2a, 0xbd0b, 0x8d68, 0x9d49, 0x7e97, 0x6eb6, 0x5ed5, 0x4ef4, 0x3e13, 0x2e32, 0x1e51, 0x0e70,
    0xff9f, 0xefbe, 0xdfdd, 0xcffc, 0xbf1b, 0xaf3a, 0x9f59, 0x8f78, 0x9188, 0x81a9, 0xb1ca, 0xa1eb,
    0xd10c, 0xc12d, 0xf14e, 0xe16f, 0x1080, 0x00a1, 0x30c2, 0x20e3, 0x5004, 0x4025, 0x7046, 0x6067,
    0x83b9, 0x9398, 0xa3fb, 0xb3da, 0xc33d, 0xd31c, 0xe37f, 0xf35e, 0x02b1, 0x1290, 0x22f3, 0x32d2,
    0x4235, 0x5214, 0x6277, 0x7256, 0xb5ea, 0xa5cb, 0x95a8, 0x8589, 0xf56e, 0xe54f, 0xd52c, 0xc50d,
    0x34e2, 0x24c3, 0x14a0, 0x0481, 0x7466, 0x6447, 0x5424, 0x4405, 0xa7db, 0xb7fa, 0x8799, 0x97b8,
    0xe75f, 0xf77e, 0xc71d, 0xd73c, 0x26d3, 0x36f2, 0x0691, 0x16b0, 0x6657, 0x7676, 0x4615, 0x5634,
    0xd94c, 0xc96d, 0xf90e, 0xe92f, 0x99c8, 0x89e9, 0xb98a, 0xa9ab, 0x5844, 0x4865, 0x7806, 0x6827,
    0x18c0, 0x08e1, 0x3882, 0x28a3, 0xcb7d, 0xdb5c, 0xeb3f, 0xfb1e, 0x8bf9, 0x9bd8, 0xabbb, 0xbb9a,
    0x4a75, 0x5a54, 0x6a37, 0x7a16, 0x0af1, 0x1ad0, 0x2ab3, 0x3a92, 0xfd2e, 0xed0f, 0xdd6c, 0xcd4d,
    0xbdaa, 0xad8b, 0x9de8, 0x8dc9, 0x7c26, 0x6c07, 0x5c64, 0x4c45, 0x3ca2, 0x2c83, 0x1ce0, 0x0cc1,
    0xef1f, 0xff3e, 0xcf5d, 0xdf7c, 0xaf9b, 0xbfba, 0x8fd9, 0x9ff8, 0x6e17, 0x7e36, 0x4e55, 0x5e74,
    0x2e93, 0x3eb2, 0x0ed1, 0x1ef0,
];

/// Calculate CRC-16-CCITT checksum according to `EMVCo` specification
///
/// This function calculates the CRC-16 checksum using the CCITT polynomial (0x1021)
/// with initial value 0xFFFF, as required by the `EMVCo` QR Code specification.
///
/// # Arguments
/// * `data` - The string data to calculate CRC for
///
/// # Returns
/// A 4-character uppercase hexadecimal string representing the CRC
///
/// # Examples
///
/// ```
/// use ethqr_gen::crc::calculate_crc16;
///
/// let data = "000201010212020140001234567890120415534512345678901252045999530358654041.005802CN5914BEST TRANSPORT6009GUANGZHOU6304";
/// assert_eq!(calculate_crc16(data), "A9AD");
/// ```
#[must_use]
pub fn calculate_crc16(data: &str) -> String {
    let mut crc: u16 = 0xFFFF;

    for byte in data.bytes() {
        let table_index = ((crc >> 8) ^ u16::from(byte)) & 0xFF;
        crc = (crc << 8) ^ CRC16_CCITT_TABLE[table_index as usize];
    }

    // EMVCo specification requires the CRC to be formatted as 4-digit uppercase hex
    format!("{crc:04X}")
}

/// Verify CRC of QR code string according to `EMVCo` specification
///
/// This function verifies that the CRC at the end of a QR code string matches
/// the calculated CRC for the payload data.
///
/// # Arguments
/// * `qr_string` - The complete QR code string including CRC
///
/// # Returns
/// `true` if CRC is valid, `false` otherwise
///
/// # Examples
///
/// ```
/// use ethqr_gen::crc::verify_crc;
///
/// let qr_string = "000201010212020140001234567890120415534512345678901252045999530358654041.005802CN5914BEST TRANSPORT6009GUANGZHOU6304A9AD";
/// assert!(verify_crc(qr_string));
/// ```
#[must_use]
pub fn verify_crc(qr_string: &str) -> bool {
    if qr_string.len() < 8 {
        return false;
    }

    // CRC should be the last 8 characters: 6304XXXX (tag 63, length 04, 4-char CRC)
    let crc_tag_start = qr_string.len().saturating_sub(8);

    // Verify CRC tag format
    if &qr_string[crc_tag_start..crc_tag_start + 4] != "6304" {
        return false;
    }

    // Extract the provided CRC
    let provided_crc = &qr_string[crc_tag_start + 4..];

    // Calculate CRC for everything except the CRC value itself
    let payload_for_crc = &qr_string[..crc_tag_start + 4];
    let calculated_crc = calculate_crc16(payload_for_crc);

    // Compare CRCs (case-insensitive)
    provided_crc.to_uppercase() == calculated_crc.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_calculation() {
        // Test with known EMVCo example data
        let test_data = "000201010212020140001234567890120415534512345678901252045999530358654041.005802CN5914BEST TRANSPORT6009GUANGZHOU6304";
        let crc = calculate_crc16(test_data);

        // Verify CRC format
        assert_eq!(crc.len(), 4);
        assert!(crc.chars().all(|c| c.is_ascii_hexdigit()));

        // Test the complete verification process
        let complete_qr = format!("{test_data}{crc}");
        assert!(verify_crc(&complete_qr));
    }

    #[test]
    fn test_crc_verification() {
        // Test with simple data
        let payload = "00020101121502014000123456789012345260116304";
        let crc = calculate_crc16(payload);
        let complete_qr = format!("{payload}{crc}");

        // Should pass CRC verification
        assert!(verify_crc(&complete_qr));

        // Corrupted QR should fail verification
        let mut corrupted = complete_qr.clone();
        corrupted.pop(); // Remove last character
        corrupted.push('X'); // Replace with wrong character
        assert!(!verify_crc(&corrupted));
    }

    #[test]
    fn test_empty_and_invalid_input() {
        // Empty string should fail
        assert!(!verify_crc(""));

        // Too short string should fail
        assert!(!verify_crc("1234567"));

        // String without proper CRC tag should fail
        assert!(!verify_crc("12345678901234567890"));
    }
}
