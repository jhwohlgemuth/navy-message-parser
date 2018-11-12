extern crate message_parser;

use message_parser::{get_subject, open};

#[test]
fn can_get_message_subject() {
    let nav18255 = open("data/NAVADMIN/NAV18255.txt");
    let subject18255 = get_subject(&nav18255);
    assert_eq!(
        subject18255,
        "2018-2019 NAVY INFLUENZA VACCINATION AND REPORTING POLICY"
    );
    let nav18256 = open("data/NAVADMIN/NAV18256.txt");
    let subject18256 = get_subject(&nav18256);
    assert_eq!(
        subject18256,
        "FISCAL YEAR 2019 CYBERSECURITY AWARENESS CHALLENGE"
    );
    let nav18257 = open("data/NAVADMIN/NAV18257.txt");
    let subjectnav18257 = get_subject(&nav18257);
    assert_eq!(subjectnav18257, "2018 ARMY-NAVY GAME SPIRIT SPOTS");
    let nav18258 = open("data/NAVADMIN/NAV18258.txt");
    let subjectnav18258 = get_subject(&nav18258);
    assert_eq!(
        subjectnav18258,
        "100TH ANNIVERSARY OF THE END OF WORLD WAR I (WWI) PLANNING ORDER"
    );
    let nav18261 = open("data/NAVADMIN/NAV18261.txt");
    let subjectnav18261 = get_subject(&nav18261);
    assert_eq!(
        subjectnav18261,
        "NAVY RESERVE PROMOTIONS TO THE PERMANENT GRADES OF CAPTAIN, COMMANDER, \nLIEUTENANT COMMANDER, LIEUTENANT, AND CHIEF WARRANT OFFICERS IN THE LINE AND \nSTAFF CORPS"
    );
    let nav18262 = open("data/NAVADMIN/NAV18262.txt");
    let subjectnav18262 = get_subject(&nav18262);
    assert_eq!(
        subjectnav18262,
        "SENATE CONFIRMATION OF OFFICERS SELECTED BY THE FY-19 NAVY ACTIVE-DUTY \nLIEUTENANT COMMANDER LINE AND STAFF CORPS BOARDS"
    );
    let nav18263 = open("data/NAVADMIN/NAV18263.txt");
    let subjectnav18263 = get_subject(&nav18263);
    assert_eq!(subjectnav18263, "UPDATE TO NAVY GRADUATE EDUCATION PROGRAM");
    let nav18265 = open("data/NAVADMIN/NAV18265.txt");
    let subjectnav18265 = get_subject(&nav18265);
    assert_eq!(
        subjectnav18265,
        "REINSTATEMENT OF THE REQUIREMENT TO DISPLAY THE OFFICER PHOTOGRAPH \nDURING SELECTION BOARDS"
    );
    let nav18268 = open("data/NAVADMIN/NAV18268.txt");
    let subjectnav18268 = get_subject(&nav18268);
    assert_eq!(
        subjectnav18268,
        "REMOVAL OF AUTOMATIC CANCELLATION PROVISION AND REVISION OF AGE \nREQUIREMENT OF INSTRUCTIONS"
    );
    let nav18269 = open("data/NAVADMIN/NAV18269.txt");
    let subjectnav18269 = get_subject(&nav18269);
    assert_eq!(
        subjectnav18269,
        "POLICY REMINDER CONCERNING POLITICAL CAMPAIGNS AND ELECTIONS"
    );
    let nav18271 = open("data/NAVADMIN/NAV18271.txt");
    let subjectnav18271 = get_subject(&nav18271);
    assert_eq!(
        subjectnav18271,
        "FY-20 NAVY ACTIVE DUTY AND RESERVE ENLISTED ADVANCEMENT SELECTION BOARDS \nFOR MASTER CHIEF AND SENIOR CHIEF PETTY OFFICER"
    );
}
#[test]
fn can_handle_bad_subject() {
    // SUBJ:
    let nav16135 = open("data/NAVADMIN/NAV16135.txt");
    let subject16135 = get_subject(&nav16135);
    assert_eq!(
        subject16135,
        "MASTER CHIEF PETTY OFFICER OF THE NAVY SELECTION"
    );
}
#[test]
fn can_handle_personal_for_message() {
    // subject line missing "//"
    let nav16180 = open("data/NAVADMIN/NAV16180.txt");
    let subject16180 = get_subject(&nav16180);
    assert_eq!(subject16180, "UNKNOWN");
}
