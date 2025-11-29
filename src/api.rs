use std::time::Duration;

use reqwest::Client;

use crate::models::{Chip, MinerData, Slot, SystemInfo};

const TIMEOUT_SECS: u64 = 30;

pub async fn fetch(ip: &str, user: &str, pass: &str) -> Result<MinerData, String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| e.to_string())?;

    // Authenticate
    let resp = client
        .post(format!("https://{ip}/cgi-bin/luci"))
        .form(&[("luci_username", user), ("luci_password", pass)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() && !resp.status().is_redirection() {
        return Err(format!("Login failed: {}", resp.status()));
    }

    // Fetch API data
    let resp = client
        .get(format!("https://{ip}/cgi-bin/luci/admin/status/btminerapi"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("API failed: {}", resp.status()));
    }

    let html = resp.text().await.map_err(|e| e.to_string())?;
    parse_html(&html)
}

pub async fn fetch_system_info(ip: &str, user: &str, pass: &str) -> Result<SystemInfo, String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| e.to_string())?;

    // Authenticate
    let resp = client
        .post(format!("https://{ip}/cgi-bin/luci"))
        .form(&[("luci_username", user), ("luci_password", pass)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() && !resp.status().is_redirection() {
        return Err(format!("Login failed: {}", resp.status()));
    }

    // Fetch overview page
    let resp = client
        .get(format!("https://{ip}/cgi-bin/luci/admin/status/overview"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Overview fetch failed: {}", resp.status()));
    }

    let html = resp.text().await.map_err(|e| e.to_string())?;
    Ok(parse_overview_html(&html))
}

fn parse_overview_html(html: &str) -> SystemInfo {
    SystemInfo {
        model: extract_table_value(html, "Model").unwrap_or_default(),
        hardware_info: extract_table_value(html, "Hardware Info").unwrap_or_default(),
        firmware_version: extract_table_value(html, "Firmware Version").unwrap_or_default(),
    }
}

fn extract_table_value(html: &str, label: &str) -> Option<String> {
    // Find pattern: <td ...>Label</td><td>VALUE</td>
    let pattern = format!(">{label}</td><td>");
    let start = html.find(&pattern)? + pattern.len();
    let end = start + html[start..].find("</td>")?;
    Some(html[start..end].to_string())
}

fn parse_html(html: &str) -> Result<MinerData, String> {
    let start = html.find(r#"id="syslog">"#).ok_or("Missing textarea")? + 12;
    let end = start
        + html[start..]
            .find("</textarea>")
            .ok_or("Unclosed textarea")?;
    parse_text(&html[start..end])
}

fn parse_text(text: &str) -> Result<MinerData, String> {
    let mut slots = Vec::new();
    let mut current: Option<Slot> = None;

    for line in text.lines().map(str::trim) {
        if line.starts_with("slot:") {
            if let Some(slot) = current.take() {
                slots.push(slot);
            }
            current = Some(parse_slot_header(line));
        } else if line.starts_with("nonce valid:") {
            if let Some(slot) = &mut current {
                parse_nonce_line(line, slot);
            }
        } else if line.starts_with('C')
            && line.contains("freq:")
            && let Some(slot) = &mut current
            && let Some(chip) = parse_chip_line(line)
        {
            slot.chips.push(chip);
        }
    }

    if let Some(slot) = current {
        slots.push(slot);
    }

    if slots.is_empty() {
        return Err("No slots found".into());
    }

    Ok(MinerData { slots })
}

fn parse_slot_header(line: &str) -> Slot {
    let mut slot = Slot::default();
    for part in line.split(',').map(str::trim) {
        if let Some((key, val)) = part.split_once(':') {
            let val = val.trim();
            match key.trim() {
                "slot" => slot.id = val.parse().unwrap_or_default(),
                "freq" => slot.freq = val.parse().unwrap_or_default(),
                "temp" => slot.temp = val.parse().unwrap_or_default(),
                "step" => slot.step = val.parse().unwrap_or_default(),
                _ => {}
            }
        }
    }
    slot
}

fn parse_nonce_line(line: &str, slot: &mut Slot) {
    // Parse "nonce valid: 981367(3182/s), ..."
    if let Some(rest) = line.strip_prefix("nonce valid:")
        && let Some(paren) = rest.find('(')
    {
        slot.nonce_valid = rest[..paren].trim().parse().unwrap_or_default();
        if let Some(slash) = rest.find("/s)") {
            slot.nonce_rate = rest[paren + 1..slash].parse().unwrap_or_default();
        }
    }

    for part in line.split(',').map(str::trim) {
        if let Some((key, val)) = part.split_once(':') {
            match key.trim() {
                "err" => slot.errors = val.trim().parse().unwrap_or_default(),
                "crc" => slot.crc = val.trim().parse().unwrap_or_default(),
                _ => {}
            }
        }
    }
}

fn parse_chip_line(line: &str) -> Option<Chip> {
    let id_end = line.find(char::is_whitespace)?;
    let id: i32 = line[1..id_end].parse().ok()?;

    let mut chip = Chip {
        id,
        ..Default::default()
    };

    // Parse "pct: 98.8%/ 94.1%"
    if let Some(pct_str) = line.split("pct:").nth(1) {
        let parts: Vec<_> = pct_str.split('/').collect();
        if let Some(p1) = parts.first() {
            chip.pct1 = p1.trim().trim_end_matches('%').parse().unwrap_or_default();
        }
        if let Some(p2) = parts.get(1) {
            chip.pct2 = p2.trim().trim_end_matches('%').parse().unwrap_or_default();
        }
    }

    for part in line.split_whitespace() {
        if let Some((key, val)) = part.split_once(':') {
            match key {
                "freq" => chip.freq = val.parse().unwrap_or_default(),
                "vol" => chip.vol = val.parse().unwrap_or_default(),
                "temp" => chip.temp = val.parse().unwrap_or_default(),
                "nonce" => chip.nonce = val.parse().unwrap_or_default(),
                "err" => chip.errors = val.parse().unwrap_or_default(),
                "crc" => chip.crc = val.parse().unwrap_or_default(),
                "x" => chip.x = val.parse().unwrap_or_default(),
                "repeat" => chip.repeat = val.parse().unwrap_or_default(),
                _ => {}
            }
        }
    }

    Some(chip)
}
