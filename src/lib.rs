// Here we have a struct in which the first field has a &str that is tied to a lifetimie, 
// but the second argument is generic, so it does not have any lifetimes.
struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

// Here, we're implemening a constructor for a StrSplit
impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Here we have a trait for a Delimiter that takes a string 
// to search into and retuns the start and end of the substrings given the 
// delimiter
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// Here we implement the Iterator trait for our struct, 
// but we require the generic type D to implement the Delimiter trait
impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delim = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

// Here we have an implementation of the Delimiter trait for a &str
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// Here we have an implementation of the Delimiter trait for a char
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

// here we have a convenient function that takes a string and a character 
// and returns a string up to that character
pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("strsplit always gives at least one result")
}

// some tests
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
