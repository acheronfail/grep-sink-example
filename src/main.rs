use grep::matcher::{Match, Matcher};
use grep::searcher::{Searcher, SearcherBuilder, Sink, SinkMatch};

struct SimpleSink {}

impl Sink for SimpleSink {
    type Error = std::io::Error;
    fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch) -> Result<bool, Self::Error> {
        println!("SINK: received match: {:#?}", mat);
        Ok(true)
    }
}

struct SimpleMatcher<'a> {
    needle: &'a [u8],
}

impl<'a> SimpleMatcher<'a> {
    pub fn new(needle: &'a [u8]) -> SimpleMatcher {
        SimpleMatcher { needle }
    }
}

impl<'a> Matcher for SimpleMatcher<'a> {
    type Captures = grep::matcher::NoCaptures;
    type Error = grep::matcher::NoError;

    fn find_at(&self, haystack: &[u8], at: usize) -> Result<Option<Match>, Self::Error> {
        println!("MATCHER: searching haystack: {:?}", haystack);
        if let Some(start) = haystack[at..]
            .windows(self.needle.len())
            .position(|window| window == self.needle)
        {
            println!(
                "MATCHER: found match at {}..{}",
                start,
                start + self.needle.len()
            );
            Ok(Some(Match::new(start, start + self.needle.len())))
        } else {
            Ok(None)
        }
    }

    fn new_captures(&self) -> Result<Self::Captures, Self::Error> {
        Ok(grep::matcher::NoCaptures::new())
    }
}

// "foo" in UTF-16
const NEEDLE: &[u8] = &[102, 0, 111, 0, 111, 0];
// echo 'foo' | iconv -f utf-8 -t utf16
const HAYSTACK: &[u8] = &[255, 254, 102, 0, 111, 0, 111, 0, 10, 0];

fn main() {
    let mut searcher = SearcherBuilder::new()
        .line_number(true)
        // Disable BOM sniffing (keeps BOM in the output)
        .bom_sniffing(false)
        // Disable encoding (pass through raw bytes)
        .encoding(None)
        .build();

    let sink = SimpleSink {};
    let matcher = SimpleMatcher::new(NEEDLE);

    searcher.search_slice(matcher, HAYSTACK, sink).unwrap();
}
