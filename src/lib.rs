#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

//! Rust library for modifying macOS tags

use std::{collections::HashSet, fmt, io, path::Path};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
/// Represents a macOS tag
pub enum Tag {
    /// Gray tag color
    Gray,
    /// Green tag color
    Green,
    /// Purple tag color
    Purple,
    /// Blue tag color
    Blue,
    /// Yellow tag color
    Yellow,
    /// Red tag color
    Red,
    /// Orange tag color
    Orange,
    /// Custom tag name (uncolored)
    Custom(String),
}

#[derive(Error, Debug)]
/// Represents an error that can occur when working with tags
pub enum TagError {
    /// Error when working with extended attributes
    #[error("xattr operation failed")]
    XAttr(#[from] io::Error),
    /// Error when working with plist data
    #[error("plist operation failed")]
    Plist(#[from] plist::Error),
    /// Error when user-provided data is invalid
    #[error("tag metadata for `{0}` is invalid")]
    Invalid(String),
    /// Unknown error
    #[error("unknown error")]
    Unknown,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Gray => write!(f, "Gray\n1"),
            Self::Green => write!(f, "Green\n2"),
            Self::Purple => write!(f, "Purple\n3"),
            Self::Blue => write!(f, "Blue\n4"),
            Self::Yellow => write!(f, "Yellow\n5"),
            Self::Red => write!(f, "Red\n6"),
            Self::Orange => write!(f, "Orange\n7"),
            Self::Custom(val) => write!(f, "{}", val),
        }
    }
}

impl Tag {
    /// Converts a `String` into a `Tag`
    pub fn from_string(s: &str) -> Result<Self, TagError> {
        if s.contains('\n') {
            let tag = s
                .split_once('\n')
                .ok_or_else(|| TagError::Invalid(s.to_owned()))?;

            match tag {
                ("Gray", "1") => return Ok(Tag::Gray),
                ("Green", "2") => return Ok(Tag::Green),
                ("Purple", "3") => return Ok(Tag::Purple),
                ("Blue", "4") => return Ok(Tag::Blue),
                ("Yellow", "5") => return Ok(Tag::Yellow),
                ("Red", "6") => return Ok(Tag::Red),
                ("Orange", "7") => return Ok(Tag::Orange),
                _ => return Err(TagError::Invalid(s.to_owned())),
            }
        }

        Ok(Tag::Custom(s.to_string()))
    }
}

/// Adds a tag for provided file
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use macos_tags::{add_tag, Tag};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let p = Path::new("./readme.md");
///     add_tag(p, Tag::Green)?;
///     Ok(())
/// }
/// ```
pub fn add_tag(path: &Path, tag: Tag) -> Result<HashSet<Tag>, TagError> {
    let tag_metadata =
        xattr::get(path, "com.apple.metadata:_kMDItemUserTags").map_err(TagError::XAttr)?;

    let parsed_tags = match tag_metadata {
        Some(t) => plist::from_bytes::<plist::Value>(&t).map_err(TagError::Plist)?,
        None => plist::Value::Array(vec![]),
    };

    match parsed_tags {
        plist::Value::Array(t) => {
            // Converting into HashSet because `dedup` doesn't seem to work
            let mut existing_tag_set = t.iter().fold(HashSet::new(), |mut acc, x| {
                if let plist::Value::String(s) = x {
                    acc.insert(s.to_owned());
                }
                acc
            });

            existing_tag_set.insert(tag.to_string());

            let tags_to_set = &existing_tag_set
                .iter()
                .map(|t| plist::Value::String(t.to_string()))
                .collect::<Vec<_>>();

            let final_tag_set = existing_tag_set
                .iter()
                .map(|t| Tag::from_string(t))
                .collect::<Result<HashSet<_>, TagError>>()?;

            let mut binary_buffer: Vec<u8> = vec![];
            plist::to_writer_binary(&mut binary_buffer, &tags_to_set).map_err(TagError::Plist)?;
            xattr::set(path, "com.apple.metadata:_kMDItemUserTags", &binary_buffer)
                .map_err(TagError::XAttr)?;
            Ok(final_tag_set)
        }
        _ => Err(TagError::Unknown),
    }
}

/// Sets all tags for provided file
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use macos_tags::{set_tags, Tag};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let p = Path::new("./readme.md");
///     set_tags(p, [Tag::Green, Tag::Red].into())?;
///     Ok(())
/// }
/// ```
pub fn set_tags(path: &Path, tags: HashSet<Tag>) -> Result<HashSet<Tag>, TagError> {
    let tags_to_set = &tags
        .iter()
        .map(|t| plist::Value::String(t.to_string()))
        .collect::<Vec<_>>();

    let mut binary_buffer: Vec<u8> = vec![];
    plist::to_writer_binary(&mut binary_buffer, &tags_to_set).map_err(TagError::Plist)?;
    xattr::set(path, "com.apple.metadata:_kMDItemUserTags", &binary_buffer)
        .map_err(TagError::XAttr)?;

    Ok(tags)
}

/// Removes a tag for provided file
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use macos_tags::{remove_tag, Tag};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let p = Path::new("./readme.md");
///     remove_tag(p, Tag::Green)?;
///     Ok(())
/// }
/// ```
pub fn remove_tag(path: &Path, tag: Tag) -> Result<HashSet<Tag>, TagError> {
    let tag_metadata =
        xattr::get(path, "com.apple.metadata:_kMDItemUserTags").map_err(TagError::XAttr)?;

    let parsed_tags = match tag_metadata {
        Some(t) => plist::from_bytes::<plist::Value>(&t).map_err(TagError::Plist)?,
        None => plist::Value::Array(vec![]),
    };

    match parsed_tags {
        plist::Value::Array(t) => {
            // Converting into HashSet because `dedup` doesn't seem to work
            let mut existing_tag_set = t.iter().fold(HashSet::new(), |mut acc, x| {
                if let plist::Value::String(s) = x {
                    acc.insert(s.to_owned());
                }
                acc
            });

            existing_tag_set.remove(&tag.to_string());

            let tags_to_set = &existing_tag_set
                .iter()
                .map(|t| plist::Value::String(t.to_string()))
                .collect::<Vec<_>>();

            let final_tag_set = existing_tag_set
                .iter()
                .map(|t| Tag::from_string(t))
                .collect::<Result<HashSet<_>, TagError>>()?;

            let mut binary_buffer: Vec<u8> = vec![];
            plist::to_writer_binary(&mut binary_buffer, &tags_to_set).map_err(TagError::Plist)?;
            xattr::set(path, "com.apple.metadata:_kMDItemUserTags", &binary_buffer)
                .map_err(TagError::XAttr)?;
            Ok(final_tag_set)
        }
        _ => Err(TagError::Unknown),
    }
}

/// Prunes all tags for provided file
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use macos_tags::{prune_tags, Tag};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let p = Path::new("./readme.md");
///     prune_tags(p)?;
///     Ok(())
/// }
/// ```
pub fn prune_tags(path: &Path) -> Result<HashSet<Tag>, TagError> {
    xattr::remove(path, "com.apple.metadata:_kMDItemUserTags").map_err(TagError::XAttr)?;
    Ok(HashSet::<Tag>::with_capacity(0))
}

/// Reads tags for provided file
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use macos_tags::{read_tags, Tag};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let p = Path::new("./readme.md");
///     read_tags(p)?;
///     Ok(())
/// }
/// ```
pub fn read_tags(path: &Path) -> Result<HashSet<Tag>, TagError> {
    let tag_metadata =
        xattr::get(path, "com.apple.metadata:_kMDItemUserTags").map_err(TagError::XAttr)?;

    let existing_tags = match tag_metadata {
        Some(t) => plist::from_bytes::<plist::Value>(&t).map_err(TagError::Plist)?,
        None => plist::Value::Array(vec![]),
    };

    match existing_tags {
        plist::Value::Array(t) => {
            let parsed_tags: HashSet<Tag> = t
                .iter()
                .filter_map(|t| {
                    if let plist::Value::String(s) = t {
                        let tag = Tag::from_string(s)
                            .map_err(|_| TagError::Invalid(s.to_owned()))
                            .ok()?;
                        Some(tag)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<Tag>>();

            Ok(parsed_tags)
        }
        _ => Err(TagError::Unknown),
    }
}
