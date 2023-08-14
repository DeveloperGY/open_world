#[derive(Clone, Copy)]
pub struct Error {
    pub err: &'static str,
    pub line: usize,
    pub filename: &'static str
}

impl Error {
    pub fn new() -> Self {
        let err = "";
        let line = 0;
        let filename = "";

        Self {
            err,
            line,
            filename
        }
    }

    pub fn set_err(&mut self, err: &'static str) {
        self.err = err;
    }

    pub fn set_fileline(&mut self, filename: &'static str, line: usize) {
        self.filename = filename;
        self.line = line;
    }
}

/// You must pass a gm::Result type to gmerr!
/// 
/// gmerr handles the generation of the line and filename for gm::Error
macro_rules! gmerr {
    ($res: expr) => {
        match $res {
            Ok(o) => o,
            Err(mut e) => { // A gm::Error is expected here which is why the gm::Result type is required
                e.set_fileline(file!(), line!() as usize);
                return Err(e);
            }
        }
    };
}

pub(crate) use gmerr;

pub type Result<T> = std::result::Result<T, Error>;