// here we're saying that, both the remainder and the delimiter pointers
// must live for at least as long as the StrSplit struct does, so we never point to an invalid
// pointer
#[derive(Debug)]
struct StrSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str
}

// here, we're saying that for any type, the haystack and the delimiter lifetimes
// must be at least as big as the StrSplit lifetime
impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

// Here we're saying the same as above, but now, the string inside the 
// iterator Item is tied to the lifetime of the StrSplit struct
impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }

        } else {
            None
        }
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("strsplit always gives at least one result")
} 

#[test]
fn works() {
    let str_split = StrSplit::new("a b c d", " ");
    let letters: Vec<_> = str_split.collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn works_when_delimiter_is_at_string_tail() {
    let str_split = StrSplit::new("a b c d ", " ");
    let letters: Vec<_> = str_split.collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
