use reqwest::Client;
use std::time::Duration;

use crate::models::{Chip, MinerData, Slot};

const REQUEST_TIMEOUT: u64 = 30;

pub async fn fetch(ip: &str, user: &str, pass: &str) -> Result<MinerData, String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_store(true)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()
        .map_err(|e| e.to_string())?;

    // Authenticate
    let login_url = format!("https://{ip}/cgi-bin/luci");
    let resp = client
        .post(&login_url)
        .form(&[("luci_username", user), ("luci_password", pass)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() && !resp.status().is_redirection() {
        return Err(format!("Login failed: {}", resp.status()));
    }

    // Fetch data
    let api_url = format!("https://{ip}/cgi-bin/luci/admin/status/btminerapi");
    let resp = client
        .get(&api_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("API failed: {}", resp.status()));
    }

    let html = resp.text().await.map_err(|e| e.to_string())?;
    parse_html(&html)
}

fn parse_html(html: &str) -> Result<MinerData, String> {
    let start = html.find(r#"id="syslog">"#).ok_or("Missing textarea")? + 12;
    let end = html[start..]
        .find("</textarea>")
        .ok_or("Unclosed textarea")?
        + start;

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
            if let Some(ref mut slot) = current {
                parse_nonce_line(line, slot);
            }
        } else if line.starts_with('C')
            && line.contains("freq:")
            && let Some(ref mut slot) = current
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
                "slot" => slot.id = val.parse().unwrap_or(0),
                "freq" => slot.freq = val.parse().unwrap_or(0),
                "temp" => slot.temp = val.parse().unwrap_or(0.0),
                "step" => slot.step = val.parse().unwrap_or(0),
                _ => {}
            }
        }
    }
    slot
}

fn parse_nonce_line(line: &str, slot: &mut Slot) {
    // Extract nonce count and rate from "nonce valid: 981367(3182/s), ..."
    if let Some(colon) = line.find(':') {
        let rest = &line[colon + 1..];
        if let Some(paren) = rest.find('(') {
            slot.nonce_valid = rest[..paren].trim().parse().unwrap_or(0);
            if let Some(slash) = rest.find("/s)") {
                slot.nonce_rate = rest[paren + 1..slash].parse().unwrap_or(0);
            }
        }
    }

    // Extract err and crc
    for part in line.split(',').map(str::trim) {
        if let Some((key, val)) = part.split_once(':') {
            match key.trim() {
                "err" => slot.errors = val.trim().parse().unwrap_or(0),
                "crc" => slot.crc = val.trim().parse().unwrap_or(0),
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

    // Parse pct: value separately since it has special format "pct: 98.8%/ 94.1%"
    if let Some(pct_idx) = line.find("pct:") {
        let pct_str = &line[pct_idx + 4..];
        let pct_str = pct_str.trim();
        // Parse "98.8%/ 94.1%" or similar
        let parts: Vec<&str> = pct_str.split('/').collect();
        if let Some(p1) = parts.first() {
            chip.pct1 = p1.trim().trim_end_matches('%').parse().unwrap_or(0.0);
        }
        if let Some(p2) = parts.get(1) {
            chip.pct2 = p2.trim().trim_end_matches('%').parse().unwrap_or(0.0);
        }
    }

    for part in line.split_whitespace() {
        if let Some((key, val)) = part.split_once(':') {
            match key {
                "freq" => chip.freq = val.parse().unwrap_or(0),
                "vol" => chip.vol = val.parse().unwrap_or(0),
                "temp" => chip.temp = val.parse().unwrap_or(0),
                "nonce" => chip.nonce = val.parse().unwrap_or(0),
                "err" => chip.errors = val.parse().unwrap_or(0),
                "crc" => chip.crc = val.parse().unwrap_or(0),
                "x" => chip.x = val.parse().unwrap_or(0),
                "repeat" => chip.repeat = val.parse().unwrap_or(0),
                _ => {}
            }
        }
    }

    Some(chip)
}
