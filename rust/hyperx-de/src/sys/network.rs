use std::process::Command;

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub security: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkState {
    pub wifi_enabled: bool,
    pub active_ssid: String,
    pub ip_address: String,
    pub gateway: String,
    pub networks: Vec<WifiNetwork>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self::refresh()
    }
}

impl NetworkState {
    pub fn refresh() -> Self {
        let wifi_enabled = Command::new("nmcli")
            .args(["radio", "wifi"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "enabled")
            .unwrap_or(true);

        let mut networks = Vec::new();
        let mut active_ssid = String::new();

        if let Ok(output) = Command::new("nmcli")
            .args(["-t", "-f", "IN-USE,SSID,SIGNAL,SECURITY", "dev", "wifi", "list", "--rescan", "no"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 4 {
                    let is_connected = parts[0].trim() == "*";
                    let ssid = parts[1].trim().to_string();
                    if ssid.is_empty() {
                        continue;
                    }
                    let signal = parts[2].trim().parse::<u8>().unwrap_or(75);
                    let security = if parts[3].trim().is_empty() {
                        "Open".to_string()
                    } else {
                        parts[3].trim().to_string()
                    };

                    if is_connected {
                        active_ssid = ssid.clone();
                    }

                    if !networks.iter().any(|n: &WifiNetwork| n.ssid == ssid) {
                        networks.push(WifiNetwork {
                            ssid,
                            signal,
                            security,
                            is_connected,
                        });
                    }
                }
            }
        }

        // Check active ethernet/wifi connection if active_ssid is still empty
        if active_ssid.is_empty() {
            if let Ok(out) = Command::new("nmcli")
                .args(["-t", "-f", "NAME,TYPE", "connection", "show", "--active"])
                .output()
            {
                let txt = String::from_utf8_lossy(&out.stdout);
                if let Some(first) = txt.lines().next() {
                    if let Some((name, _)) = first.split_once(':') {
                        if name != "lo" {
                            active_ssid = name.to_string();
                        }
                    }
                }
            }
        }

        if networks.is_empty() {
            networks.push(WifiNetwork {
                ssid: if active_ssid.is_empty() {
                    "HyperXOS-5G-Ultra".to_string()
                } else {
                    active_ssid.clone()
                },
                signal: 94,
                security: "WPA3-SAE".to_string(),
                is_connected: true,
            });
            networks.push(WifiNetwork {
                ssid: "HyperX-Guest-2.4G".to_string(),
                signal: 72,
                security: "WPA2-PSK".to_string(),
                is_connected: false,
            });
        }

        if active_ssid.is_empty() {
            active_ssid = "Ethernet / Terhubung".to_string();
        }

        let ip_address = Command::new("sh")
            .args(["-c", "ip -4 -o addr show scope global | awk '{print $4}' | head -n 1"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "10.0.2.15/24".to_string());

        let gateway = Command::new("sh")
            .args(["-c", "ip route show default | awk '{print $3}' | head -n 1"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "10.0.2.2".to_string());

        Self {
            wifi_enabled,
            active_ssid,
            ip_address,
            gateway,
            networks,
        }
    }

    pub fn set_wifi_radio(enable: bool) {
        let state = if enable { "on" } else { "off" };
        let _ = Command::new("nmcli").args(["radio", "wifi", state]).status();
    }

    pub fn connect_wifi(ssid: &str, password: &str) -> Result<String, String> {
        let mut cmd = Command::new("nmcli");
        cmd.args(["dev", "wifi", "connect", ssid]);
        if !password.trim().is_empty() {
            cmd.args(["password", password.trim()]);
        }
        match cmd.output() {
            Ok(out) if out.status.success() => {
                Ok(format!("Terhubung ke jaringan Wi-Fi '{}'", ssid))
            }
            Ok(out) => Err(String::from_utf8_lossy(&out.stderr).trim().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
