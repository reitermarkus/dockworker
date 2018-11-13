use byteorder::{BigEndian, ReadBytesExt};
use hyper::client::response::Response;
use std;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub command: String,
    pub created: u64,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub size_rw: Option<u64>, // I guess it is optional on Mac.
    pub size_root_fs: Option<u64>,
    pub labels: Option<HashMap<String, String>>,
    pub host_config: HostConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub port_type: String, // Renamed because `type` is a keyword.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub network_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    pub config: Config,
    pub created: String,
    pub driver: String,
    // ExecIDs
    // GraphDriver
    // HostConfig
    pub hostname_path: String,
    pub hosts_path: String,
    pub id: String,
    pub image: String,
    pub log_path: String,
    pub mount_label: String,
    pub mounts: Vec<Mount>,
    pub name: String,
    pub network_settings: NetworkSettings,
    pub path: String,
    pub process_label: String,
    pub resolv_conf_path: String,
    pub restart_count: u64,
    pub state: State,
}

/// This type represents a `struct{}` in the Go code.
pub type UnspecifiedObject = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    // TODO: Verify that this is never just a `String`.
    //pub Cmd: Vec<String>,
    pub domainname: String,
    // TODO: The source says `Option<String>` but I've seen
    // `Option<Vec<String>>` on the wire.  Ignore until we figure it out.
    //pub Entrypoint: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub exposed_ports: Option<HashMap<String, UnspecifiedObject>>,
    pub hostname: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    // TODO: We don't know exacly what this vec contains.
    //pub OnBuild: Option<Vec<???>>,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub user: String,
    pub volumes: Option<HashMap<String, UnspecifiedObject>>,
    pub working_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    // Name (optional)
    // Driver (optional)
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub propagation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub bridge: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u32,
    pub hairpin_mode: bool,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: u32,
    pub mac_address: String,
    pub networks: HashMap<String, Network>,
    pub ports: Option<HashMap<String, Option<Vec<PortMapping>>>>,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    pub sandbox_key: String,
    // These two are null in the current output.
    //pub SecondaryIPAddresses: ,
    //pub SecondaryIPv6Addresses: ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u32,
    //pub IPAMConfig: ,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    //pub Links:
    pub mac_address: String,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortMapping {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub dead: bool,
    // I don't know whether PIDs can be negative here.  They're normally
    // positive, but sometimes negative PIDs are used in certain APIs.
    pub pid: i64,
    pub exit_code: i64,
    pub error: String,
    pub started_at: String,
    pub finished_at: String,
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}

impl std::fmt::Display for ContainerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize)]
pub struct ContainerFilters {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    id: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    name: Vec<String>,
}

impl Default for ContainerFilters {
    fn default() -> Self {
        Self {
            id: vec![],
            name: vec![],
        }
    }
}

impl ContainerFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id.push(id.to_owned());
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name.push(name.to_owned());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ContainerStdioType {
    Stdin,
    Stdout,
    Stderr,
}

/// response fragment of the attach container api
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AttachResponseFrame {
    type_: ContainerStdioType,
    frame: Vec<u8>,
}

impl AttachResponseFrame {
    fn new(type_: ContainerStdioType, frame: Vec<u8>) -> Self {
        Self { type_, frame }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.frame
    }
}

#[derive(Debug, Clone)]
struct ContainerStdio {
    /// io type
    type_: ContainerStdioType,
    /// shared source (response)
    src: Rc<RefCell<AttachResponseIter>>,
    stdin_buff: Rc<RefCell<Vec<u8>>>,
    stdout_buff: Rc<RefCell<Vec<u8>>>,
    stderr_buff: Rc<RefCell<Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct ContainerStdin {
    body: ContainerStdio,
}

#[derive(Debug, Clone)]
pub struct ContainerStdout {
    body: ContainerStdio,
}

#[derive(Debug, Clone)]
pub struct ContainerStderr {
    body: ContainerStdio,
}

impl ContainerStdin {
    fn new(body: ContainerStdio) -> Self {
        Self { body }
    }
}

impl ContainerStdout {
    fn new(body: ContainerStdio) -> Self {
        Self { body }
    }
}

impl ContainerStderr {
    fn new(body: ContainerStdio) -> Self {
        Self { body }
    }
}

impl Read for ContainerStdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.body.read(buf)
    }
}

impl Read for ContainerStdout {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.body.read(buf)
    }
}

impl Read for ContainerStderr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.body.read(buf)
    }
}

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

impl ContainerStdio {
    fn new(
        type_: ContainerStdioType,
        src: Rc<RefCell<AttachResponseIter>>,
        stdin_buff: Rc<RefCell<Vec<u8>>>,
        stdout_buff: Rc<RefCell<Vec<u8>>>,
        stderr_buff: Rc<RefCell<Vec<u8>>>,
    ) -> Self {
        Self {
            type_,
            src,
            stdin_buff,
            stdout_buff,
            stderr_buff,
        }
    }

    fn forcused_buff(&self) -> Ref<Vec<u8>> {
        use container::ContainerStdioType::*;
        match self.type_ {
            Stdin => self.stdin_buff.borrow(),
            Stdout => self.stdout_buff.borrow(),
            Stderr => self.stderr_buff.borrow(),
        }
    }

    fn forcused_buff_mut(&self) -> RefMut<Vec<u8>> {
        use container::ContainerStdioType::*;
        match self.type_ {
            Stdin => self.stdin_buff.borrow_mut(),
            Stdout => self.stdout_buff.borrow_mut(),
            Stderr => self.stderr_buff.borrow_mut(),
        }
    }

    // read next chunk from response to the inner buffer
    fn readin_next(&mut self) -> io::Result<usize> {
        use container::ContainerStdioType::*;

        while let Some(xs) = self.src.borrow_mut().next() {
            let AttachResponseFrame { type_, mut frame } = xs?;
            let len = frame.len();
            match type_ {
                Stdin => self.stdin_buff.borrow_mut().append(&mut frame),
                Stdout => self.stdout_buff.borrow_mut().append(&mut frame),
                Stderr => self.stderr_buff.borrow_mut().append(&mut frame),
            }
            if type_ == self.type_ {
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

/// Response of attach to container api
#[derive(Debug)]
pub struct AttachResponse {
    res: Response,
}

impl AttachResponse {
    pub fn new(res: Response) -> Self {
        Self { res }
    }
}

impl From<AttachResponse> for AttachContainer {
    fn from(res: AttachResponse) -> Self {
        let iter = Rc::new(RefCell::new(res.res.into())); // into_iter
        let stdin_buff = Rc::new(RefCell::new(Vec::new()));
        let stdout_buff = Rc::new(RefCell::new(Vec::new()));
        let stderr_buff = Rc::new(RefCell::new(Vec::new()));
        let stdin = ContainerStdin::new(ContainerStdio::new(
            ContainerStdioType::Stdin,
            Rc::clone(&iter),
            Rc::clone(&stdin_buff),
            Rc::clone(&stdout_buff),
            Rc::clone(&stderr_buff),
        ));
        let stdout = ContainerStdout::new(ContainerStdio::new(
            ContainerStdioType::Stdout,
            Rc::clone(&iter),
            Rc::clone(&stdin_buff),
            Rc::clone(&stdout_buff),
            Rc::clone(&stderr_buff),
        ));
        let stderr = ContainerStderr::new(ContainerStdio::new(
            ContainerStdioType::Stderr,
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
        use container::ContainerStdioType::*;

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
            0 => Some(Ok(AttachResponseFrame::new(Stdin, frame))),
            1 => Some(Ok(AttachResponseFrame::new(Stdout, frame))),
            2 => Some(Ok(AttachResponseFrame::new(Stderr, frame))),
            n => {
                error!("unexpected kind of chunk: {}", n);
                None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExitStatus {
    status_code: i32,
}

impl ExitStatus {
    pub fn new(status_code: i32) -> Self {
        Self {
            status_code: status_code,
        }
    }

    pub fn into_inner(self) -> i32 {
        self.status_code
    }
}

impl From<i32> for ExitStatus {
    fn from(status_code: i32) -> Self {
        Self::new(status_code)
    }
}
