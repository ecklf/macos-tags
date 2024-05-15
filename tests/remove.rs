use macos_tags::{self, add_tag, prune_tags, read_tags, remove_tag, Tag};
use std::error::Error;
use test_utilities::run_file_test;

#[test]
fn removes_single_tag() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        add_tag(p, Tag::Red)?;
        remove_tag(p, Tag::Red)?;
        let tags = read_tags(p)?;
        assert_eq!(tags, vec![Tag::Green]);
        Ok(())
    })
}

#[test]
fn prunes_tags() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        add_tag(p, Tag::Red)?;
        prune_tags(p)?;
        let tags = read_tags(p)?;
        assert_eq!(tags, vec![]);
        Ok(())
    })
}
