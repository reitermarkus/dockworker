use std::cell::{Ref, RefCell, RefMut};
use std::io::{self, Read};
use std::rc::Rc;

use byteorder::{BigEndian, ReadBytesExt};
use hyper::client::response::Response;

use models::AttachResponse;

#[derive(Debug)]
pub struct AttachContainer {
  pub stdin: ContainerStdin,
  pub stdout: ContainerStdout,
  pub stderr: ContainerStderr,
}

impl AttachContainer {
  fn new(stdin: ContainerStdin, stdout: ContainerStdout, stderr: ContainerStderr) -> Self {
    Self {
      stdin,
      stdout,
      stderr,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ContainerIoType {
  Stdin,
  Stdout,
  Stderr,
}

#[derive(Debug, Clone)]
pub struct ContainerStdin {
  body: ContainerStdio,
}

impl ContainerStdin {
  fn new(body: ContainerStdio) -> Self {
    Self { body }
  }
}

impl Read for ContainerStdin {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.body.read(buf)
  }
}

#[derive(Debug, Clone)]
pub struct ContainerStdout {
  body: ContainerStdio,
}

impl ContainerStdout {
  fn new(body: ContainerStdio) -> Self {
    Self { body }
  }
}

impl Read for ContainerStdout {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.body.read(buf)
  }
}

#[derive(Debug, Clone)]
pub struct ContainerStderr {
  body: ContainerStdio,
}

impl ContainerStderr {
  fn new(body: ContainerStdio) -> Self {
    Self { body }
  }
}

impl Read for ContainerStderr {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.body.read(buf)
  }
}

#[derive(Debug, Clone)]
struct ContainerStdio {
  /// io type
  io_type: ContainerIoType,
  /// shared source (response)
  src: Rc<RefCell<AttachResponseIter>>,
  stdin_buff: Rc<RefCell<Vec<u8>>>,
  stdout_buff: Rc<RefCell<Vec<u8>>>,
  stderr_buff: Rc<RefCell<Vec<u8>>>,
}

impl ContainerStdio {
  fn new(
    io_type: ContainerIoType,
    src: Rc<RefCell<AttachResponseIter>>,
    stdin_buff: Rc<RefCell<Vec<u8>>>,
    stdout_buff: Rc<RefCell<Vec<u8>>>,
    stderr_buff: Rc<RefCell<Vec<u8>>>,
  ) -> Self {
    Self {
      io_type,
      src,
      stdin_buff,
      stdout_buff,
      stderr_buff,
    }
  }

  fn forcused_buff(&self) -> Ref<Vec<u8>> {
    match self.io_type {
      ContainerIoType::Stdin => self.stdin_buff.borrow(),
      ContainerIoType::Stdout => self.stdout_buff.borrow(),
      ContainerIoType::Stderr => self.stderr_buff.borrow(),
    }
  }

  fn forcused_buff_mut(&self) -> RefMut<Vec<u8>> {
    match self.io_type {
      ContainerIoType::Stdin => self.stdin_buff.borrow_mut(),
      ContainerIoType::Stdout => self.stdout_buff.borrow_mut(),
      ContainerIoType::Stderr => self.stderr_buff.borrow_mut(),
    }
  }

  // read next chunk from response to the inner buffer
  fn readin_next(&mut self) -> io::Result<usize> {
    while let Some(xs) = self.src.borrow_mut().next() {
      let AttachResponseFrame { io_type, mut frame } = xs?;
      let len = frame.len();
      match io_type {
        ContainerIoType::Stdin => self.stdin_buff.borrow_mut().append(&mut frame),
        ContainerIoType::Stdout => self.stdout_buff.borrow_mut().append(&mut frame),
        ContainerIoType::Stderr => self.stderr_buff.borrow_mut().append(&mut frame),
      }
      if io_type == self.io_type {
        return Ok(len);
      }
    }

    Ok(0) // end of stream
  }
}

impl Read for ContainerStdio {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    if self.forcused_buff().len() == 0 {
      match self.readin_next() {
        Ok(0) => return Ok(0),
        Err(e) => return Err(e),
        _ => {}
      }
    }
    let inner_buf_len = self.forcused_buff().len(); // > 0

    if inner_buf_len <= buf.len() {
      debug!("{} <= {}", inner_buf_len, buf.len());
      buf[..inner_buf_len].copy_from_slice(&self.forcused_buff()); // copy
      self.forcused_buff_mut().clear(); // clear inner buffer
      Ok(inner_buf_len)
    } else {
      // inner_buf_len > buf.len()
      debug!("{} > {}", inner_buf_len, buf.len());
      let buf_len = buf.len();
      buf.copy_from_slice(&self.forcused_buff()[..buf_len]); // copy (fill buf)
      let mut inner_buf = self.forcused_buff_mut();
      inner_buf.drain(..buf_len); // delete _size_ elementes from the head of buf
      Ok(buf_len)
    }
  }
}

impl From<AttachResponse> for AttachContainer {
  fn from(res: AttachResponse) -> Self {
    let iter = Rc::new(RefCell::new(res.res.into())); // into_iter
    let stdin_buff = Rc::new(RefCell::new(Vec::new()));
    let stdout_buff = Rc::new(RefCell::new(Vec::new()));
    let stderr_buff = Rc::new(RefCell::new(Vec::new()));
    let stdin = ContainerStdin::new(ContainerStdio::new(
      ContainerIoType::Stdin,
      Rc::clone(&iter),
      Rc::clone(&stdin_buff),
      Rc::clone(&stdout_buff),
      Rc::clone(&stderr_buff),
    ));
    let stdout = ContainerStdout::new(ContainerStdio::new(
      ContainerIoType::Stdout,
      Rc::clone(&iter),
      Rc::clone(&stdin_buff),
      Rc::clone(&stdout_buff),
      Rc::clone(&stderr_buff),
    ));
    let stderr = ContainerStderr::new(ContainerStdio::new(
      ContainerIoType::Stderr,
      Rc::clone(&iter),
      Rc::clone(&stdin_buff),
      Rc::clone(&stdout_buff),
      Rc::clone(&stderr_buff),
    ));
    AttachContainer::new(stdin, stdout, stderr)
  }
}

#[derive(Debug)]
struct AttachResponseIter {
  res: Response,
}

impl AttachResponseIter {
  fn new(res: Response) -> Self {
    Self { res }
  }
}

impl From<Response> for AttachResponseIter {
  fn from(res: Response) -> Self {
    Self::new(res)
  }
}

impl Iterator for AttachResponseIter {
  type Item = io::Result<AttachResponseFrame>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut buf = [0u8; 8];
    // read header
    if let Err(err) = self.res.read_exact(&mut buf) {
      return if err.kind() == io::ErrorKind::UnexpectedEof {
        None // end of stream
      } else {
        Some(Err(err))
      };
    }
    // read body
    let mut frame_size_raw = &buf[4..];
    let frame_size = frame_size_raw.read_u32::<BigEndian>().unwrap();
    let mut frame = vec![0; frame_size as usize];
    if let Err(io) = self.res.read_exact(&mut frame) {
      return Some(Err(io));
    }
    match buf[0] {
      0 => Some(Ok(AttachResponseFrame::new(ContainerIoType::Stdin, frame))),
      1 => Some(Ok(AttachResponseFrame::new(ContainerIoType::Stdout, frame))),
      2 => Some(Ok(AttachResponseFrame::new(ContainerIoType::Stderr, frame))),
      n => {
        error!("unexpected kind of chunk: {}", n);
        None
      }
    }
  }
}

/// response fragment of the attach container api
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct AttachResponseFrame {
  io_type: ContainerIoType,
  frame: Vec<u8>,
}

impl AttachResponseFrame {
  fn new(io_type: ContainerIoType, frame: Vec<u8>) -> Self {
    Self { io_type, frame }
  }
}

