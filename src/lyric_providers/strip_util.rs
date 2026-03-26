//! Handle extended timestamp tags (e.g., `<mm:ss.ms>`) in lyrics.
//!
//! This module provides functionality to identify and remove extended timestamp tags
//! from a lyrics string. The core function [`strip_extended_timestamps`] efficiently
//! scans the string, deletes tags matching a specific `<...>` pattern, and preserves
//! all other content.
//!
//! The extended timestamp format is defined as:
//! - Starts with `<` and ends with `>`;
//! - The inner part follows the pattern `mm:ss.ms`, where:
//!   - The minute part consists of 1-2 digits,
//!   - The second part consists of 1-2 digits,
//!   - The millisecond part consists of 1-3 digits.
//! - The total length (including angle brackets) is between 7 and 11 characters.

use std::borrow::Cow;

use memchr::memchr;

/// Checks whether the given byte slice matches the extended timestamp tag format (e.g., `<mm:ss.ms>`).
///
/// Format requirements:
/// - Starts with `<` and ends with `>`;
/// - The inner part follows `mm:ss.ms`, where:
///   - The minute part has 1-2 digits,
///   - The second part has 1-2 digits,
///   - The millisecond part has 1-3 digits;
/// - The total length (including angle brackets) is between 7 and 11 bytes.
///
/// # Arguments
/// * `buf` - The byte slice to inspect.
///
/// # Returns
/// `true` if `buf` exactly matches the format, `false` otherwise.
#[inline(always)]
fn is_extended_tag(buf: &[u8]) -> bool {
    let len = buf.len();
    // Quick length filter: total length must be between 7 and 11 bytes (including brackets)
    if !(7..=11).contains(&len) {
        return false;
    }

    // Check first and last characters are angle brackets
    if buf[0] != b'<' || buf[len - 1] != b'>' {
        return false;
    }

    // Inner part without angle brackets
    let content = &buf[1..len - 1];

    let mut pos = 0;

    // Minute part, followed by ':'
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=2).contains(&(pos - start)) || pos == content.len() || content[pos] != b':' {
        return false;
    }
    pos += 1;

    // Second part, followed by '.'
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=2).contains(&(pos - start)) || pos == content.len() || content[pos] != b'.' {
        return false;
    }
    pos += 1;

    // Millisecond part, until the end
    let start = pos;
    while pos < content.len() && content[pos].is_ascii_digit() {
        pos += 1;
    }
    if !(1..=3).contains(&(pos - start)) || pos != content.len() {
        return false;
    }

    true
}

/// Finds the range of the next angle-bracket tag starting from the given position.
///
/// This function searches the byte slice for the next tag that starts with `<` and ends with `>`,
/// and returns its start and end indices (the end index points to the `>` character).
/// It does **not** validate the content of the tag.
///
/// # Arguments
/// * `bytes` - The complete byte slice to search.
/// * `start` - The starting index (inclusive) for the search; must be within bounds of `bytes`.
///
/// # Returns
/// * `Some((usize, usize))` - A tuple containing the start index (pointing to `<`) and end index
///   (pointing to `>`) of the tag.
/// * `None` - If no `<` is found after `start`, or if a `<` is found but no matching `>` follows.
///
/// # Note
/// This function does not handle nested tags or self-closing tags; it simply locates the simplest
/// `<...>` pattern.
#[inline(always)]
fn find_next_tag(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    // Find the first '<' byte starting from 'start'
    let tag_start = memchr(b'<', &bytes[start..])?;
    let tag_start = start + tag_start; // convert to global index

    // Skip '<' and look for the next '>'
    let after_start = tag_start + 1;
    let tag_end_offset = memchr(b'>', &bytes[after_start..])?;
    let tag_end = after_start + tag_end_offset; // convert to global index
    Some((tag_start, tag_end))
}

/// Removes all extended timestamp tags (`<mm:ss.ms>`) from a lyrics string, preserving other content.
///
/// This function employs a lazy copying strategy to avoid unnecessary allocations:
/// - If no angle brackets are found, it directly returns a borrowed reference to the original string (`Cow::Borrowed`).
/// - If tags that need to be removed are found, it builds a new string, copying only the necessary parts,
///   and avoids repeated allocations for each segment.
///
/// # Arguments
/// * `lrc` - The original lyrics string.
///
/// # Returns
/// * `Cow<'a, str>` - If the string contains no extended timestamp tags, a borrowed reference (`Borrowed`) is returned;
///   otherwise, an owned string (`Owned`) with all extended timestamp tags removed.
pub fn strip_extended_timestamps<'a>(lrc: &'a str) -> Cow<'a, str> {
    let bytes = lrc.as_bytes();
    let len = bytes.len();

    // Fast path: if no angle brackets exist, borrow directly
    if memchr(b'<', bytes).is_none() || memchr(b'>', bytes).is_none() {
        return Cow::Borrowed(lrc);
    }

    let mut pos = 0; // current scan position
    let mut result: Option<String> = None; // result being built
    let mut kept_segments: Vec<(usize, usize)> = Vec::new(); // temporarily stored retained segments

    while let Some((tag_start, tag_end)) = find_next_tag(bytes, pos) {
        let tag_slice = &lrc[tag_start..=tag_end];

        if is_extended_tag(tag_slice.as_bytes()) {
            // This tag needs to be removed
            match result.as_mut() {
                Some(res) => {
                    // Result already exists, append the text before the tag
                    res.push_str(&lrc[pos..tag_start]);
                }
                None => {
                    // First removal: create result and copy all previously kept segments
                    let mut new_res = String::with_capacity(len);
                    for &(start, end) in &kept_segments {
                        new_res.push_str(&lrc[start..end]);
                    }
                    kept_segments.clear();
                    new_res.push_str(&lrc[pos..tag_start]);
                    result = Some(new_res);
                }
            }
            pos = tag_end + 1; // skip the tag
        } else {
            // Retain this tag
            if let Some(res) = result.as_mut() {
                res.push_str(&lrc[pos..=tag_end]);
            } else {
                kept_segments.push((pos, tag_end + 1)); // store half-open interval [pos, tag_end+1)
            }
            pos = tag_end + 1;
        }
    }

    // Process the remaining part
    match result {
        Some(mut it) => {
            if pos < len {
                it.push_str(&lrc[pos..]);
            }
            Cow::Owned(it)
        }
        None => Cow::Borrowed(lrc),
    }
}
