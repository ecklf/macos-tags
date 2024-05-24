use macos_tags::{self, add_tag, read_tags, set_tags, Tag};
use std::{collections::HashSet, error::Error};
use test_utilities::run_file_test;

#[test]
fn reads_tags() -> Result<(), Box<dyn Error>> {
    run_file_test(|p| {
        add_tag(p, Tag::Green)?;
        let tags = read_tags(p)?;
        let expect: HashSet<Tag> = [Tag::Green].into();
        assert_eq!(tags, expect);
        set_tags(p, [Tag::Green, Tag::Red].into())?;
        let tags = read_tags(p)?;
        let expect: HashSet<Tag> = [Tag::Green, Tag::Red].into();
        assert_eq!(tags, expect);
        Ok(())
    })
}
