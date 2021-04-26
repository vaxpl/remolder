use std::fs::File;
use std::path::Path;

pub trait FileDigest {
    fn md5_str(&self) -> String;
}

impl<P> FileDigest for P
where
    P: AsRef<Path>,
{
    fn md5_str(&self) -> String {
        File::open(self).map_or(String::new(), |mut f| {
            let mut ctx = md5::Context::new();
            std::io::copy(&mut f, &mut ctx)
                .map_or(String::new(), |_| format!("{:x}", ctx.compute()))
        })
    }
}

pub struct TeeWriter<'a, A, B>
where
    A: std::io::Write,
    B: std::io::Write,
{
    a: &'a mut A,
    b: &'a mut B,
}

impl<'a, A, B> TeeWriter<'a, A, B>
where
    A: std::io::Write,
    B: std::io::Write,
{
    pub fn new(a: &'a mut A, b: &'a mut B) -> Self {
        Self { a, b }
    }
}

impl<'a, A, B> Drop for TeeWriter<'a, A, B>
where
    A: std::io::Write,
    B: std::io::Write,
{
    fn drop(&mut self) {
        self.a.flush().unwrap();
        self.b.flush().unwrap();
    }
}

impl<'a, A, B> std::io::Write for TeeWriter<'a, A, B>
where
    A: std::io::Write,
    B: std::io::Write,
{
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.a.flush().and(self.b.flush())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        self.a.write(data).and(self.b.write(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_digest() {
        assert!(Path::new("Cargo.toml").md5_str().len() > 0);
        assert!(Path::new("Dummy.toml").md5_str().len() == 0);
    }

    #[test]
    fn test_tee_writer() {
        use std::io::Write;

        let mut f1 = File::create("test1.txt").unwrap();
        let mut f2 = File::create("test2.txt").unwrap();
        {
            let mut w2 = TeeWriter::new(&mut f1, &mut f2);
            w2.write(b"Hello, TeeWriter\n");
        }
    }
}
