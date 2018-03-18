use std::result;
use basic::Error as BasicError;

#[derive(Debug, YadeError)]
pub enum Error {
    /// An error coming from the `basic` crate.
    Basic(BasicError),

    /// Not a valid descriptor.
    InvalidDescriptor { desc: String, at: usize },
}

pub type Result<T> = result::Result<T, Error>;
