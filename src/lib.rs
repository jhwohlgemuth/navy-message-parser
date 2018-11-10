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
#[test]
fn can_open_files() {
    assert_eq!(open("data/foo.txt"), "foo\n");
}

fn get_classification(contents: &str) -> &str {
    match take_until_break(contents) {
        Ok(i) => i.1,
        Err(_) => "UNKNOWN",
    }
}
#[test]
fn can_get_classification() {
    let contents = "UNCLASSIFIED//
                    ROUTINE
                    R 291453Z OCT 18
                    FM CNO WASHINGTON DC
                    TO NAVADMIN
                    INFO CNO WASHINGTON DC
                    ";
    assert_eq!(get_classification(contents), "UNCLASSIFIED");
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
        classification: get_classification(contents),
    }
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

pub fn get_subject(contents: &str) -> &str {
    named!(parse<&str, &str>, do_parse!(
        take_until_subj >>
        subject: take_until_break >>
        (subject)
    ));
    match parse(contents) {
        Ok(i) => &i.1[5..],
        Err(_) => "UNKNOWN"
    }
}
#[test]
fn can_get_subject() {
    let nav18255 = open("data/NAVADMIN/NAV18255.txt");
    let subject18255 = get_subject(&nav18255);
    assert_eq!(subject18255, "2018-2019 NAVY INFLUENZA VACCINATION AND REPORTING POLICY");
    let nav18256 = open("data/NAVADMIN/NAV18256.txt");
    let subject18256 = get_subject(&nav18256);
    assert_eq!(subject18256, "FISCAL YEAR 2019 CYBERSECURITY AWARENESS CHALLENGE");
}
