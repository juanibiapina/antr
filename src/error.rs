use std::result;
use std::io;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub enum Error {
    CantReadCurrentDirectory(std::rc::Rc<io::Error>),
    InvalidCurrentDirectory(std::rc::Rc<io::Error>),
}
