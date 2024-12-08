use std::{
    ffi::OsStr,
    fs, io,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchResult<T>(pub T, pub Duration);

pub fn bench<T, F: FnOnce() -> T>(f: F) -> BenchResult<T> {
    let now = Instant::now();
    let result = f();
    BenchResult(result, now.elapsed())
}

pub fn write_dir_safe<P: AsRef<OsStr>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    let path = Path::new(&path);
    fs::create_dir_all(path.parent().expect("path does not have a parent"))?;
    fs::write(path, contents)
}

pub fn goto_previous_line() {
    print!("\x1b[F\x1b[2K");
}
