#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::prelude::*;

named!(take_until_break<&str, &str>, take_until!("//"));
named!(take_until_subj<&str, &str>, take_until!("SUBJ/"));

pub fn open(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}
pub struct Header<'a> {
    pub classification: &'a str,
    // pub datetime:  &'a str,
    // pub from: &'a str,
    // pub to: &'a str,
    // pub info: &'a str,
}
pub fn get_header(contents: &str) -> Header {
    Header {
        classification: match take_until_break(contents) {
            Ok(i) => i.1,
            Err(_) => "UNKNOWN",
        },
    }
}
pub fn get_subject(contents: &str) -> &str {
    named!(parse<&str, &str>, do_parse!(
        take_until!("SUBJ") >>
        tag!("SUBJ") >>
        ws!(alt!(tag!("/") | tag!(":"))) >>
        subject: take_until_break >>
        (subject)
    ));
    match parse(contents) {
        Ok(i) => &i.1,
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
    fn can_get_header_classification() {
        let contents = "UNCLASSIFIED//
                    ROUTINE
                    R 291453Z OCT 18
                    FM CNO WASHINGTON DC
                    TO NAVADMIN
                    INFO CNO WASHINGTON DC
                    ";
        assert_eq!(get_header(contents).classification, "UNCLASSIFIED");
    }
    #[test]
    fn can_get_subject() {
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
