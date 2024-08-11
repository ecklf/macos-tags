use macos_tags::{self, add_tag, prune_tags, remove_tag, Tag};
use std::{collections::HashSet, error::Error};
use test_utilities::run_file_test;

#[test]
fn removes_single_tag() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, &Tag::Green)?;
        add_tag(p, &Tag::Red)?;
        let result = remove_tag(p, &Tag::Red)?;
        let expect: HashSet<Tag> = [Tag::Green].into();
        assert_eq!(result, expect);
        Ok(())
    })
}

#[test]
fn prunes_tags() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, &Tag::Green)?;
        add_tag(p, &Tag::Red)?;
        let result = prune_tags(p)?;
        let expect: HashSet<Tag> = [].into();
        assert_eq!(result, expect);
        Ok(())
    })
}
