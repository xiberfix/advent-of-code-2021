use std::str::FromStr;

pub trait Input {
    fn values<A: FromStr, R: FromIterator<A>>(self) -> R;
}

impl<'a, T: Iterator<Item=&'a str>> Input for T {
    fn values<A: FromStr, R: FromIterator<A>>(self) -> R {
        self.filter_map(|s| s.parse().ok()).collect()
    }
}
