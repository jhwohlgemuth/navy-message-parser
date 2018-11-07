#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::prelude::*;

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
    named!(x<&str, &str>, take_until!("//"));
    match x(contents) {
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
