use macos_tags::{self, add_tag, read_tags, set_tags, Tag};
use std::error::Error;
use test_utilities::run_file_test;

#[test]
fn adds_single_tag() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        let tags = read_tags(p)?;
        assert_eq!(tags, vec![Tag::Green]);
        // Ensure that adding a tag does not modify the existing tag(s)
        add_tag(p, Tag::Red)?;
        let tags = read_tags(p)?;
        assert!(tags.contains(&Tag::Green));
        assert!(tags.contains(&Tag::Red));
        // Ensure that adding a tag multiple times does not duplicate it
        add_tag(p, Tag::Red)?;
        let tags = read_tags(p)?;
        assert!(tags.len() == 2);
        Ok(())
    })
}

#[test]
fn sets_all_tags() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        set_tags(p, vec![Tag::Red, Tag::Orange])?;
        let tags = read_tags(p)?;
        assert!(!tags.contains(&Tag::Green));
        assert!(tags.contains(&Tag::Red));
        assert!(tags.contains(&Tag::Orange));
        Ok(())
    })
}
