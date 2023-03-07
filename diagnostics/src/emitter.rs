use parking_lot::Mutex;

use crate::term::termcolor::*;

pub trait Emitter: Send + Sync {
    fn buffer(&self) -> Buffer;
    fn print(&self, buffer: Buffer) -> std::io::Result<()>;
}

pub struct DefaultEmitter {
    writer: BufferWriter,
}
impl DefaultEmitter {
    pub fn new(color: ColorChoice) -> Self {
        let writer = BufferWriter::stderr(color);
        Self { writer }
    }
}
impl Emitter for DefaultEmitter {
    #[inline(always)]
    fn buffer(&self) -> Buffer {
        self.writer.buffer()
    }

    #[inline(always)]
    fn print(&self, buffer: Buffer) -> std::io::Result<()> {
        self.writer.print(&buffer)
    }
}

#[derive(Default)]
pub struct CaptureEmitter {
    buffer: Mutex<Vec<u8>>,
}
impl CaptureEmitter {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn captured(&self) -> String {
        let buf = self.buffer.lock();
        String::from_utf8_lossy(buf.as_slice()).into_owned()
    }
}
impl Emitter for CaptureEmitter {
    #[inline]
    fn buffer(&self) -> Buffer {
        Buffer::no_color()
    }

    #[inline]
    fn print(&self, buffer: Buffer) -> std::io::Result<()> {
        let mut bytes = buffer.into_inner();
        let mut buf = self.buffer.lock();
        buf.append(&mut bytes);
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub struct NullEmitter {
    ansi: bool,
}
impl NullEmitter {
    pub fn new(color: ColorChoice) -> Self {
        let ansi = match color {
            ColorChoice::Never => false,
            ColorChoice::Always | ColorChoice::AlwaysAnsi => true,
            ColorChoice::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    true
                } else {
                    false
                }
            }
        };
        Self { ansi }
    }
}
impl Emitter for NullEmitter {
    #[inline(always)]
    fn buffer(&self) -> Buffer {
        if self.ansi {
            Buffer::ansi()
        } else {
            Buffer::no_color()
        }
    }

    #[inline(always)]
    fn print(&self, _buffer: Buffer) -> std::io::Result<()> {
        Ok(())
    }
}
