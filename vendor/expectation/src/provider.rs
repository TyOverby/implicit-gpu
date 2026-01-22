use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use expectation_shared::filesystem::{FileSystem, ReadSeek};

pub struct WriteRequester {
    pub(crate) fs: Box<FileSystem>,
    pub(crate) files: Vec<PathBuf>,
}

impl WriteRequester {
    pub fn request<S, Fn>(&mut self, path: S, mut f: Fn) -> IoResult<()>
    where
        S: AsRef<Path>,
        Fn: for<'a> FnMut(&'a mut Write) -> IoResult<()>,
    {
        let mut v = vec![];
        v.push(1u8);
        self.files.push(self.fs.full_path_for(path.as_ref()));
        self.fs.write(path.as_ref(), &mut f)
    }
}

pub(crate) type Files = Vec<(
        PathBuf,
        Box<for<'a> Fn(&'a mut ReadSeek, &'a mut ReadSeek) -> IoResult<bool>>,
        Box<
            for<'b> Fn(&'b mut ReadSeek, &'b mut ReadSeek, &'b Path, &'b mut WriteRequester)
                -> IoResult<()>,
        >,
    )>;

pub struct Provider {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) root_fs: Box<FileSystem>,
    pub(crate) fs: Box<FileSystem>,
    pub(crate) files: Arc<Mutex<Files>>,
    cur_offset: PathBuf,
}

pub struct Writer {
    inner: Vec<u8>,
    filesystem: Box<FileSystem>,
    path: PathBuf,
}

impl Clone for Provider {
    fn clone(&self) -> Provider {
        Provider {
            root_fs: self.root_fs.duplicate(),
            fs: self.fs.duplicate(),
            files: self.files.clone(),
            cur_offset: self.cur_offset.clone(),
        }
    }
}

impl Writer {
    fn new(filesystem: Box<FileSystem>, path: PathBuf) -> Self {
        Writer {
            filesystem,
            path,
            inner: vec![],
        }
    }
}

impl Provider {
    pub fn subdir<P: AsRef<Path>>(&self, path: P) -> Provider {
        assert!(path.as_ref().is_relative(), "path is not relative");
        Provider {
            root_fs: self.root_fs.duplicate(),
            fs: self.fs.duplicate().subsystem(path.as_ref()),
            files: self.files.clone(),
            cur_offset: self.cur_offset.join(path),
        }
    }

    pub fn new(root_fs: Box<FileSystem>, fs: Box<FileSystem>) -> Provider {
        Provider {
            root_fs,
            fs,
            files: Arc::new(Mutex::new(vec![])),
            cur_offset: PathBuf::new(),
        }
    }

    pub(crate) fn take_files(&self) -> Files {
        use std::mem::swap;
        let mut empty = vec![];
        let mut lock = self.files.lock().unwrap();
        swap(&mut empty, &mut lock);
        empty
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let mut contents = Vec::new();
        ::std::mem::swap(&mut contents, &mut self.inner);
        // TODO: maybe don't ignore?
        let _ = self
            .filesystem
            .write(&self.path, &mut |w| w.write_all(&contents));
    }
}

impl Provider {
    pub fn custom_test<S, C, D>(&self, name: S, compare: C, diff: D) -> Writer
    where
        S: AsRef<Path>,
        C: for<'a> Fn(&'a mut (ReadSeek), &'a mut (ReadSeek)) -> IoResult<bool> + 'static,
        D: for<'b> Fn(&'b mut (ReadSeek), &'b mut (ReadSeek), &'b Path, &'b mut WriteRequester) -> IoResult<()>
            + 'static,
    {
        let name: PathBuf = name.as_ref().into();
        let mut lock = self.files.lock().unwrap();
        lock
            .push((
                self.cur_offset.join(name.clone()),
                Box::new(compare),
                Box::new(diff)));
        Writer::new(self.fs.duplicate(), name )
    }
}


#[test]
fn writer_does_not_write_to_filesystem_if_not_written_to() {
    use expectation_shared::filesystem::*;
    let filesystem = Box::new(FakeFileSystem::new()) as Box<FileSystem>;
    {
        let _writer = Writer::new(filesystem.duplicate(), "foo.txt".into());
    }
    assert!(filesystem.exists(Path::new("foo.txt")));
}
