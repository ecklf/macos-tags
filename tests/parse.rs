use macos_tags::{self, Tag, TagError};

#[test]
fn it_does_parse_valid_custom_tag() {
    let tag_str = "Custom";
    let res = Tag::from_string(tag_str);
    assert!(res.is_ok(), "Expected to parse valid custom tag");
    assert_eq!(res.unwrap(), Tag::Custom(tag_str.to_owned()));
}

#[test]
fn it_does_parse_valid_system_tag() {
    let tag_str = "Gray\n1";
    let res = Tag::from_string(tag_str);
    assert!(res.is_ok(), "Expected to parse valid system tag");
    assert_eq!(res.unwrap(), Tag::Gray);
}

#[test]
fn it_does_not_parse_invalid_system_name_tag() {
    let tag_str = "Green\n1";
    let res = Tag::from_string(tag_str);
    assert!(res.is_err(), "Expected not to parse invalid system tag");

    match res {
        Ok(_) => panic!("Expected not to parse invalid system tag"),
        Err(TagError::Invalid(s)) => assert!(s == tag_str),
        _ => panic!("Expected invalid error"),
    }
}

#[test]
fn it_does_not_parse_invalid_system_id_tag() {
    let tag_str = "Gray\n2";
    let res = Tag::from_string(tag_str);
    assert!(res.is_err(), "Expected not to parse invalid system tag");

    match res {
        Ok(_) => panic!("Expected not to parse invalid system tag"),
        Err(TagError::Invalid(s)) => assert!(s == tag_str),
        _ => panic!("Expected invalid error"),
    }
}

#[test]
fn it_does_not_parse_malformed_system_tag() {
    let tag_str = "Gray\n\n1";
    let res = Tag::from_string(tag_str);
    assert!(res.is_err(), "Expected not to parse invalid system tag");

    match res {
        Ok(_) => panic!("Expected not to parse invalid system tag"),
        Err(TagError::Invalid(s)) => assert!(s == tag_str),
        _ => panic!("Expected invalid error"),
    }
}
