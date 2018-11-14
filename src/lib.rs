#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::prelude::*;

named!(take_until_newline<&str, &str>, take_until!("\n"));

pub fn open(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}
pub struct Header<'a> {
    pub classification: &'a str,
    pub precedence: &'a str,
    pub dtg: &'a str,
    pub addresses: &'a str,
}
pub struct DateTimeGroup<'a> {
    pub prosign: &'a str,
    pub datetime: &'a str,
    pub month: &'a str,
    pub year: &'a str,
}
pub struct Addresses<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub info: &'a str,
}
pub fn parse_dtg(dtg: &str) -> DateTimeGroup {
    named!(parse<&str, DateTimeGroup>, do_parse!(
        prosign: ws!(take!(1)) >>
        datetime: ws!(take_until_and_consume!("Z")) >>
        month: ws!(take!(3)) >>
        year: take!(2) >>
        (DateTimeGroup {
            prosign: prosign,
            datetime: datetime,
            month: month,
            year: year
        })
    ));
    match parse(dtg) {
        Ok(i) => i.1,
        Err(_) => DateTimeGroup {
            prosign: "R",
            datetime: "UNKNOWN",
            month: "UNKNOWN",
            year: "UNKNOWN",
        },
    }
}
pub fn parse_addresses(contents: &str) -> Addresses {
    named!(get_from<&str, &str>, do_parse!(
        take_until_and_consume!("FM ") >>
        from: take_until_newline >>
        (from)
    ));
    named!(get_to<&str, &str>, do_parse!(
        take_until_and_consume!("TO ") >>
        to: take_until_newline >>
        (to)
    ));
    named!(get_info<&str, &str>, do_parse!(
        take_until_and_consume!("INFO ") >>
        info: take_until_newline >>
        (info)
    ));
    Addresses {
        from: match get_from(contents) {
            Ok(i) => i.1,
            Err(_) => "UNKNOWN",
        },
        to: match get_to(contents) {
            Ok(i) => i.1,
            Err(_) => "UNKNOWN",
        },
        info: match get_info(contents) {
            Ok(i) => i.1,
            Err(_) => "UNKNOWN",
        },
    }
}
pub fn get_header(contents: &str) -> Header {
    named!(parse<&str, Header>, do_parse!(
        classification: ws!(take_until_and_consume!("//")) >>
        precedence: ws!(take_until!("\n")) >>
        datetimegroup: ws!(take_until!("\n")) >>
        addresses: take_until_and_consume!("BT\n") >>
        (Header {
            classification: classification,
            precedence: precedence,
            dtg: datetimegroup,
            addresses: addresses
        })
    ));
    match parse(contents) {
        Ok(i) => i.1,
        Err(_) => Header {
            classification: "UNKNOWN",
            precedence: "UNKNOWN",
            dtg: "UNKNOWN",
            addresses: "UNKNOWN",
        },
    }
}
pub fn get_subject(contents: &str) -> &str {
    named!(parse<&str, &str>, do_parse!(
        take_until_and_consume!("SUBJ") >>
        ws!(alt!(tag!("/") | tag!(":"))) >>
        subject: take_until!("//") >>
        (subject)
    ));
    match parse(contents) {
        Ok(i) => i.1,
        Err(_) => "UNKNOWN",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_open_files() {
        assert_eq!(open("data/foo.txt"), "foo\n");
    }
    #[test]
    fn can_get_header() {
        let contents = "UNCLASSIFIED//
                    ROUTINE\n
                    R 291453Z OCT 18
                    FM CNO WASHINGTON DC
                    TO NAVADMIN
                    INFO CNO WASHINGTON DC

                    BT\n
                    ";
        assert_eq!(get_header(contents).classification, "UNCLASSIFIED");
        assert_eq!(get_header(contents).precedence, "ROUTINE");
    }
    #[test]
    fn can_parse_datetimegroup() {
        let contents = "R 123456Z MMM YY";
        let parsed_dtg = parse_dtg(contents);
        assert_eq!(parsed_dtg.prosign, "R");
        assert_eq!(parsed_dtg.datetime, "123456");
        assert_eq!(parsed_dtg.month, "MMM");
        assert_eq!(parsed_dtg.year, "YY");
    }
    #[test]
    fn can_get_subject() {
        let content = "UNCLASSIFIED//
                   ROUTINE
                   R 221554Z OCT 18
                   FM CNO WASHINGTON DC
                   TO NAVADMIN
                   INFO CNO WASHINGTON DC
                   BT\n
                   UNCLAS
                   PASS TO OFFICE CODES:
                   FM CNO WASHINGTON DC//N2N6//
                   INFO CNO WASHINGTON DC//N2N6//
   
                   NAVADMIN 256/18
   
                   MSGID/GENADMIN/CNO WASHINGTON DC/N2N6/OCT//
   
                   SUBJ/FISCAL YEAR 2019 CYBERSECURITY AWARENESS CHALLENGE//
                   ";
        let subject = get_subject(&content);
        assert_eq!(
            subject,
            "FISCAL YEAR 2019 CYBERSECURITY AWARENESS CHALLENGE"
        );
    }
    #[test]
    fn can_handle_missing_subject() {
        let content = "UNCLASSIFIED//
                   ROUTINE
                   R 221554Z OCT 18
                   FM CNO WASHINGTON DC
                   TO NAVADMIN
                   INFO CNO WASHINGTON DC
                   BT
                   UNCLAS
                   PASS TO OFFICE CODES:
                   FM CNO WASHINGTON DC//N2N6//
                   INFO CNO WASHINGTON DC//N2N6//
   
                   NAVADMIN 256/18
                   ";
        let subject = get_subject(&content);
        assert_eq!(subject, "UNKNOWN");
    }
}
