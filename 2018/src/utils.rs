use std::fmt;

pub struct Response<T> {
    pub tag: String,
    pub response: T,
}

impl<T: fmt::Display> fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.tag, self.response)
    }
}
