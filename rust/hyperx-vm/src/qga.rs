use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

pub const QGA_SOCKET: &str = "/tmp/qga.sock";

/// Minimal QEMU Guest Agent client used for automated install & smoke tests.
pub struct QgaClient {
    stream: UnixStream,
    reader: BufReader<UnixStream>,
    sync_id: u64,
}

impl QgaClient {
    /// Connect and synchronize with the guest agent (fails if the guest agent is not up yet).
    pub fn connect() -> Result<Self, String> {
        let stream = UnixStream::connect(QGA_SOCKET).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;
        let reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let mut client = Self {
            stream,
            reader,
            sync_id: 1000,
        };
        client.sync()?;
        Ok(client)
    }

    /// Wait until the guest agent inside the VM responds (or timeout).
    pub fn wait_ready(timeout: Duration) -> Result<Self, String> {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if let Ok(c) = Self::connect() {
                return Ok(c);
            }
            std::thread::sleep(Duration::from_secs(2));
        }
        Err(format!(
            "QEMU Guest Agent did not respond within {}s",
            timeout.as_secs()
        ))
    }

    fn sync(&mut self) -> Result<(), String> {
        self.sync_id += 1;
        let id = self.sync_id;
        // 0xFF flushes any partial command in the guest agent parser
        let _ = self.stream.write_all(&[0xff]);
        self.send(&json!({"execute": "guest-sync", "arguments": {"id": id}}))?;
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(5) {
            let v = self.read_line()?;
            if v.get("return").and_then(|r| r.as_u64()) == Some(id) {
                return Ok(());
            }
        }
        Err("guest-sync timeout".to_string())
    }

    fn send(&mut self, v: &Value) -> Result<(), String> {
        let mut s = v.to_string();
        s.push('\n');
        self.stream.write_all(s.as_bytes()).map_err(|e| e.to_string())
    }

    fn read_line(&mut self) -> Result<Value, String> {
        loop {
            let mut line = String::new();
            let n = self.reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if n == 0 {
                return Err("QGA connection closed".to_string());
            }
            // Skip stray 0xFF / non-JSON garbage
            let trimmed = line.trim_matches(|c: char| c.is_whitespace() || c == '\u{fffd}');
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
                return Ok(v);
            }
        }
    }

    fn call(&mut self, v: &Value) -> Result<Value, String> {
        self.send(v)?;
        let resp = self.read_line()?;
        if let Some(err) = resp.get("error") {
            return Err(format!("QGA error: {}", err));
        }
        Ok(resp.get("return").cloned().unwrap_or(Value::Null))
    }

    /// Run a shell command in the guest and wait for it to finish. Returns (exit_code, stdout+stderr).
    pub fn exec(&mut self, shell_cmd: &str) -> Result<(i64, String), String> {
        let ret = self.call(&json!({
            "execute": "guest-exec",
            "arguments": {"path": "/bin/sh", "arg": ["-c", shell_cmd], "capture-output": true}
        }))?;
        let pid = ret
            .get("pid")
            .and_then(|p| p.as_i64())
            .ok_or("guest-exec returned no pid")?;
        loop {
            std::thread::sleep(Duration::from_millis(300));
            let st = self.call(&json!({"execute": "guest-exec-status", "arguments": {"pid": pid}}))?;
            if st.get("exited").and_then(|e| e.as_bool()) == Some(true) {
                let code = st.get("exitcode").and_then(|c| c.as_i64()).unwrap_or(-1);
                let mut out = decode_field(&st, "out-data");
                out.push_str(&decode_field(&st, "err-data"));
                return Ok((code, out));
            }
        }
    }

    /// Launch a shell command in the background inside the guest (does not wait).
    pub fn spawn(&mut self, shell_cmd: &str) -> Result<(), String> {
        self.call(&json!({
            "execute": "guest-exec",
            "arguments": {"path": "/bin/sh", "arg": ["-c", shell_cmd]}
        }))
        .map(|_| ())
    }
}

fn decode_field(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|d| d.as_str())
        .map(|s| String::from_utf8_lossy(&base64_decode(s)).to_string())
        .unwrap_or_default()
}

/// Tiny standard base64 decoder (avoids pulling an extra crate).
pub fn base64_decode(input: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let mut out = Vec::with_capacity(input.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u32;
    for &c in input.as_bytes() {
        if let Some(v) = val(c) {
            buf = (buf << 6) | v;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push((buf >> bits) as u8);
                buf &= (1 << bits) - 1;
            }
        }
    }
    out
}
