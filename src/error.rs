use notify;
use std::io;
use std::path::PathBuf;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub enum Error {
    CantReadCurrentDirectory(std::rc::Rc<io::Error>),
    DebouncerInitializationError(std::rc::Rc<notify::Error>),
    InvalidCurrentDirectory(std::rc::Rc<io::Error>),
    ReadEntryError(std::rc::Rc<io::Error>),
    WatcherError(PathBuf, std::rc::Rc<notify::Error>),
}
