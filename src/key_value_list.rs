#[derive(Debug)]
pub struct KeyValuePair<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

/// Key Value List Iterator
/// Inside the gel there are some files that contains key value pairs, for example commits.
/// Since multiple things have key value paris like this with separated with space and new line,
/// it made sense to create an iterator and use that iterator in every place that we have these.
/// Key value pairs raw string is like this:
/// "key1 value1 content\nkey2 value2 content\n"
/// That becomes:
/// KeyValuePair { key: "key1", value: "value1 content" },
/// KeyValuePair { key: "key2", value: "value2 content" }
///
/// We can also have multi line values like this as well
/// "key1 multi line value\n second line of the value \n third line\nkey2 second value\n"
/// We know that the value continues because there is an extra space after each new line("\n "). This becomes:
/// KeyValuePair { key: "key1", value: "multi line value\n second line of the value \n third line" },
/// KeyVa:luePair { key: "key2", value: "second value" }
///
/// Also we can have "message" content at the end. This message content is a special case and doesn't contain any key.
/// "key1 value1 content\n\nmessage content, we have no key!\n". Note the double new line("\n\n"). This becomes:
/// KeyValuePair { key: "key1", value: "value1 content" },
/// KeyValuePair { key: "", value: "message content, we have no key!" }
///
/// FIXME: We need to find a way to remove those spaces after each new lines while parsing.
#[derive(Debug)]
pub struct KVLIterator<'a> {
    raw: &'a str,
    pos: usize,
}

impl<'a> KVLIterator<'a> {
    pub fn new(raw: &'a str) -> Self {
        KVLIterator { raw, pos: 0 }
    }
}

impl<'a> Iterator for KVLIterator<'a> {
    type Item = KeyValuePair<'a>;

    fn next(&mut self) -> Option<KeyValuePair<'a>> {
        // note: this len() will give us wrong length if there is utf-8 chars
        // inside since it counts bytes instead of graphemes. But this is raw
        // string from file, we want bytes anyway.
        let raw_length = self.raw.len();

        if self.pos == raw_length {
            return None;
        }

        let start = self.pos;
        let byte_slice = self.raw[start..].as_bytes();
        let space = byte_slice
            .iter()
            .position(|&a| a == b' ')
            .map(|x| start + x);
        let new_line = byte_slice
            .iter()
            .position(|&a| a == b'\n')
            .map(|x| start + x)
            .unwrap();

        if space.is_none() || new_line < space.unwrap() {
            assert!(new_line == start);
            self.pos = raw_length;

            return Some(KeyValuePair {
                key: "",
                value: &self.raw[(start + 1)..],
            });
            // List should be finished
        }

        let key = &self.raw[start..space.unwrap()];

        // We need to recursively find out the value.
        let mut end = start;
        loop {
            let next_slice = self.raw[(end)..].as_bytes();
            let new_line = next_slice
                .iter()
                .position(|&a| a == b'\n')
                .map(|x| end + x)
                .unwrap();

            end = new_line + 1;
            if self.raw.bytes().nth(end) != Some(b' ') {
                break;
            }
        }

        self.pos = end;

        // hmm, we need to find out a way to replace "\n " with "\n"
        Some(KeyValuePair {
            key,
            value: &self.raw[(space.unwrap() + 1)..(end - 1)], // end - 1 because removing the last \n.
        })
    }
}
