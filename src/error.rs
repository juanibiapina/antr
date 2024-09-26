use std::result;
use std::io;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub enum Error {
    InvalidCurrentDirectory(std::rc::Rc<io::Error>),
}
