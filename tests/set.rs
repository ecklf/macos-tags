use macos_tags::{self, add_tag, set_tags, Tag};
use std::{collections::HashSet, error::Error};
use test_utilities::run_file_test;

#[test]
fn adds_single_tag() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        let result = add_tag(p, Tag::Green)?;
        let expect: HashSet<Tag> = [Tag::Green].into();
        assert_eq!(result, expect);
        // Ensure that adding a tag does not modify the existing tag(s)
        let result = add_tag(p, Tag::Red)?;
        assert!(result.contains(&Tag::Green));
        assert!(result.contains(&Tag::Red));
        // Ensure that adding a tag multiple times does not duplicate it
        let result = add_tag(p, Tag::Red)?;
        assert!(result.len() == 2);
        Ok(())
    })
}

#[test]
fn sets_all_tags() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        let result = set_tags(p, [Tag::Red, Tag::Orange].into())?;
        assert!(!result.contains(&Tag::Green));
        assert!(result.contains(&Tag::Red));
        assert!(result.contains(&Tag::Orange));
        Ok(())
    })
}
