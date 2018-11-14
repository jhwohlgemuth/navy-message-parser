extern crate message_parser;

use message_parser::{get_header, open, parse_addresses, parse_dtg};

#[test]
fn can_parse_message_header() {
    let nav18255 = open("data/NAVADMIN/NAV18255.txt");
    let header18255 = get_header(&nav18255);
    assert_eq!(header18255.classification, "UNCLASSIFIED");
    assert_eq!(header18255.precedence, "ROUTINE");
    assert_eq!(header18255.dtg, "R 181847Z OCT 18");
    let parsed_addresses = parse_addresses(header18255.addresses);
    assert_eq!(parsed_addresses.from, "CNO WASHINGTON DC");
    assert_eq!(parsed_addresses.to, "NAVADMIN");
    assert_eq!(parsed_addresses.info, "CNO WASHINGTON DC");
    //
    // Personal For Message (no from/to/info)
    //
    let nav16180 = open("data/NAVADMIN/NAV16180.txt");
    let header16180 = get_header(&nav16180);
    let parsed_bad_addresses = parse_addresses(header16180.addresses);
    assert_eq!(parsed_bad_addresses.from, "UNKNOWN");
    assert_eq!(parsed_bad_addresses.to, "UNKNOWN");
    assert_eq!(parsed_bad_addresses.info, "UNKNOWN");
}
#[test]
fn can_parse_datetimegroup() {
    let message = open("data/NAVADMIN/NAV18255.txt");
    let header = get_header(&message);
    let dtg = header.dtg;
    let parsed_dtg = parse_dtg(dtg);
    assert_eq!(parsed_dtg.prosign, "R");
    assert_eq!(parsed_dtg.datetime, "181847");
    assert_eq!(parsed_dtg.month, "OCT");
    assert_eq!(parsed_dtg.year, "18");
}
// #[test]
// fn can_handle_personal_for_message() {
//     // subject line missing "//"
//     let nav16180 = open("data/NAVADMIN/NAV16180.txt");
//     let header16180 = get_header(&nav16180);
//     assert_eq!(header16180.classification, "UNCLASSIFIED");
// }
