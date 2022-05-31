use std::io::{BufWriter, Write};

pub enum Value {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(),
    Array(Vec<Value>),
}

pub struct CommandWriter {
    buf: Vec<u8>,
}

pub struct Command {
    args: Vec<String>,
    len: usize,
}

impl Command {
    pub fn new(cmd: &str, mut args: Vec<String>) -> Self {
        Self {
            args: {
                args.insert(0, cmd.to_string());
                args
            },
            len: 1,
        }
    }
}

impl CommandWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write(&mut self, cmd: &Command) -> &Vec<u8> {
        self.add_char('*');
        self.add_string(&cmd.len.to_string());
        self.add_crln();

        for arg in &cmd.args {
            self.add_char('$');
            self.add_string(&arg.len().to_string());
            self.add_crln();
            self.add_string(&arg);
            self.add_crln();
        }
        &self.buf
    }

    fn add_char(&mut self, c: char) -> &Self {
        self.buf.push(c as u8);
        self
    }
    fn add_string(&mut self, s: &str) -> &Self {
        self.buf.extend_from_slice(s.as_bytes());
        self
    }
    fn add_crln(&mut self) -> &Self {
        self.buf.extend_from_slice("\r\n".as_bytes());
        self
    }
}
