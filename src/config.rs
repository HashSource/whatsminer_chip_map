/// Miner hardware configuration data extracted from WhatsMiner firmware
/// Format: (model, chip_num, chips_per_domain, board_num)
#[allow(dead_code)]
pub struct MinerConfig {
    pub model: &'static str,
    pub chip_num: u16,
    pub chips_per_domain: u8,
    pub board_num: u8,
}

#[allow(dead_code)]
impl MinerConfig {
    /// Calculate domains per board
    pub const fn domains_per_board(&self) -> u16 {
        self.chip_num / self.chips_per_domain as u16
    }

    /// Calculate chips per board
    pub const fn chips_per_board(&self) -> u16 {
        self.chip_num / self.board_num as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_whatsminer_m50s_vh55() {
        // This is the format from the HTML API
        let result = lookup("WhatsMiner M50S_VH55");
        assert!(result.is_some(), "Should find config for M50S_VH55");
        let cfg = result.unwrap();
        assert_eq!(
            cfg.chips_per_domain, 3,
            "M50S should have 3 chips per domain"
        );
        println!(
            "Found: {} with {} chips, {} chips/domain",
            cfg.model, cfg.chip_num, cfg.chips_per_domain
        );
    }

    #[test]
    fn test_lookup_exact_match() {
        let result = lookup("M50SVH50");
        assert!(result.is_some());
        assert_eq!(result.unwrap().model, "M50SVH50");
    }

    #[test]
    fn test_lookup_m50s_plusplus_vk40() {
        // Test the M50S++ model with underscore separator
        let result = lookup("WhatsMiner M50S++_VK40");
        assert!(result.is_some(), "Should find config for M50S++_VK40");
        let cfg = result.unwrap();
        assert_eq!(cfg.model, "M50S++VK40");
        println!(
            "Found: {} with {} chips, {} chips/domain",
            cfg.model, cfg.chip_num, cfg.chips_per_domain
        );
    }

    #[test]
    fn test_lookup_m50s_plusplus_hardware_string() {
        // Test full hardware info string
        let result = lookup("M50S++_VK40.H616-CB6V10.P222B-VE1-197806A");
        assert!(result.is_some(), "Should find config from hardware string");
        let cfg = result.unwrap();
        assert_eq!(cfg.model, "M50S++VK40");
    }
}

/// Normalize model string: uppercase, keep alphanumeric and '+', strip "WHATSMINER" prefix
fn normalize_model(model: &str) -> String {
    let upper: String = model.to_uppercase();
    // Keep alphanumeric and '+' (for M50S++ style models)
    let filtered: String = upper
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '+')
        .collect();
    filtered
        .strip_prefix("WHATSMINER")
        .unwrap_or(&filtered)
        .to_string()
}

/// Lookup miner config by model name (flexible matching)
pub fn lookup(model: &str) -> Option<&'static MinerConfig> {
    let normalized = normalize_model(model);

    // Try exact match first (normalized input contains config model)
    if let Some(cfg) = CONFIGS.iter().find(|c| normalized.contains(c.model)) {
        return Some(cfg);
    }

    // Try finding config where config model starts with same base
    // e.g., input "M50SVH55" should match "M50SVH50" (same base M50SVH)
    // Extract base model by finding longest common prefix
    for prefix_len in (4..=normalized.len()).rev() {
        let prefix = &normalized[..prefix_len];
        if let Some(cfg) = CONFIGS.iter().find(|c| c.model.starts_with(prefix)) {
            return Some(cfg);
        }
    }

    // Try matching just the series (M50S, M60S, etc.)
    if let Some(series_end) = normalized.find(['V', '+']) {
        let series = &normalized[..series_end];
        if let Some(cfg) = CONFIGS.iter().find(|c| c.model.starts_with(series)) {
            return Some(cfg);
        }
    }

    None
}

/// All known miner configurations
pub static CONFIGS: &[MinerConfig] = &[
    // M30 Series
    MinerConfig {
        model: "M30KV10",
        chip_num: 240,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M30LV10",
        chip_num: 144,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M30S++V10",
        chip_num: 255,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M30S++V20",
        chip_num: 255,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M30S++VE30",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VE40",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VE50",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VF40",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VG30",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VG40",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VG50",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH10",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH100",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH110",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH20",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH30",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH40",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH50",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH60",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH70",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH80",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VH90",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VI30",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VJ20",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VJ30",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VJ50",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VJ60",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VJ70",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S++VK30",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 2,
    },
    MinerConfig {
        model: "M30S++VK40",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V100",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V10",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V20",
        chip_num: 255,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V40",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V50",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V60",
        chip_num: 245,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V70",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V80",
        chip_num: 245,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+V90",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VE30",
        chip_num: 148,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VE40",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VE50",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VE60",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VF20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VF30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VG20",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VG30",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VG40",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VG50",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VG60",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH10",
        chip_num: 64,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH20",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH30",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH40",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH50",
        chip_num: 64,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH60",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VH70",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VI30",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VJ30",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30S+VJ40",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV10",
        chip_num: 148,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV20",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV30",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV40",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV50",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV60",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SV80",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVE10",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVE20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVE30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVE40",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVE50",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVF10",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVF20",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVF30",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVG10",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVG20",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVG30",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVG40",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVH10",
        chip_num: 64,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVH20",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVH40",
        chip_num: 64,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVH50",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVH60",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVI20",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M30SVJ30",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30V10",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M30V20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    // M31 Series
    MinerConfig {
        model: "M31HV10",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31HV40",
        chip_num: 136,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M31LV10",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SEV10",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SEV20",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SEV30",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V100",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V10",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V40",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V50",
        chip_num: 148,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V60",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V80",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+V90",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VE10",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VE20",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VE30",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VE40",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VE50",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VF20",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VG20",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31S+VG30",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV10",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV50",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV60",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SV90",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M31SVE10",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31V10",
        chip_num: 70,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M31V20",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    // M32/M33 Series
    MinerConfig {
        model: "M32V10",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M32V20",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M33S++VG40",
        chip_num: 174,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M33S++VH20",
        chip_num: 112,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M33S+VG20",
        chip_num: 112,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M33S+VG30",
        chip_num: 162,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M33S+VH20",
        chip_num: 100,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M33SVG30",
        chip_num: 116,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M33V10",
        chip_num: 33,
        chips_per_domain: 1,
        board_num: 3,
    },
    MinerConfig {
        model: "M33V20",
        chip_num: 62,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M33V30",
        chip_num: 66,
        chips_per_domain: 2,
        board_num: 3,
    },
    // M34/M36/M39 Series
    MinerConfig {
        model: "M34S+VE10",
        chip_num: 116,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M36S++VH30",
        chip_num: 80,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M36S+VG30",
        chip_num: 108,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M36SVE10",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M39V10",
        chip_num: 50,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M39V20",
        chip_num: 54,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M39V30",
        chip_num: 68,
        chips_per_domain: 2,
        board_num: 3,
    },
    // M50 Series
    MinerConfig {
        model: "M50S++VK10",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VK20",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VK30",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VK40",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VK50",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VK60",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL10",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL20",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL30",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL40",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL50",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S++VL60",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VH30",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VH40",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VJ30",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VJ40",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VJ60",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VK10",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VK20",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VK30",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VL10",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VL20",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50S+VL30",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVH20",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVH30",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVH40",
        chip_num: 148,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVH50",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVJ10",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVJ20",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVJ30",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVJ40",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVJ50",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK10",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK50",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK60",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK70",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVK80",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVL10",
        chip_num: 74,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVL20",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50SVL30",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VE30",
        chip_num: 255,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M50VG30",
        chip_num: 156,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH10",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH40",
        chip_num: 84,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH50",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH60",
        chip_num: 84,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH70",
        chip_num: 105,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH80",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VH90",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VJ10",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VJ20",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VJ30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VJ40",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VJ60",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VK40",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M50VK50",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    // M51/M52/M53 Series
    MinerConfig {
        model: "M51S+VL30",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M52S++VL10",
        chip_num: 87,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M52SVK30",
        chip_num: 62,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53HVH10",
        chip_num: 56,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VK10",
        chip_num: 198,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VK20",
        chip_num: 192,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VK30",
        chip_num: 240,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VK50",
        chip_num: 186,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VL10",
        chip_num: 128,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S++VL30",
        chip_num: 174,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S+VJ30",
        chip_num: 240,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S+VJ40",
        chip_num: 248,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S+VJ50",
        chip_num: 264,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M53S+VK30",
        chip_num: 168,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53SVH20",
        chip_num: 198,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53SVH30",
        chip_num: 204,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53SVJ30",
        chip_num: 180,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53SVJ40",
        chip_num: 192,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53SVK30",
        chip_num: 128,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53VH30",
        chip_num: 128,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53VH40",
        chip_num: 174,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53VH50",
        chip_num: 162,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M53VK30",
        chip_num: 100,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M53VK60",
        chip_num: 100,
        chips_per_domain: 2,
        board_num: 4,
    },
    // M54/M56 Series
    MinerConfig {
        model: "M54S++VK30",
        chip_num: 96,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M54S++VL30",
        chip_num: 68,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M54S++VL40",
        chip_num: 90,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M54S+VL30",
        chip_num: 84,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M54SVH30",
        chip_num: 120,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M54SVK30",
        chip_num: 102,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S++VK10",
        chip_num: 160,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S++VK30",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S++VK40",
        chip_num: 132,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S++VK50",
        chip_num: 152,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S+VJ30",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S+VK30",
        chip_num: 108,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S+VK40",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56S+VK50",
        chip_num: 120,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56SVH30",
        chip_num: 152,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56SVJ30",
        chip_num: 132,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M56SVJ40",
        chip_num: 152,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M56VH30",
        chip_num: 108,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M59VH30",
        chip_num: 132,
        chips_per_domain: 3,
        board_num: 4,
    },
    // M60 Series
    MinerConfig {
        model: "M60S++VL10",
        chip_num: 204,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VL30",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VL40",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VL50",
        chip_num: 245,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VL70",
        chip_num: 294,
        chips_per_domain: 6,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VM30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VM40",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VM50",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VM60",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S++VM70",
        chip_num: 141,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VK30",
        chip_num: 245,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VK40",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M60S+VK50",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M60S+VK60",
        chip_num: 294,
        chips_per_domain: 6,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VK70",
        chip_num: 306,
        chips_per_domain: 6,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL100",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL10",
        chip_num: 196,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL30",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL40",
        chip_num: 188,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL50",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL60",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL70",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL80",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VL90",
        chip_num: 184,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VM20",
        chip_num: 82,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VM30",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VM40",
        chip_num: 90,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60S+VM50",
        chip_num: 98,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK10",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK20",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK30",
        chip_num: 245,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK40",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK60",
        chip_num: 188,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK70",
        chip_num: 196,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK80",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVK90",
        chip_num: 192,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL10",
        chip_num: 147,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL20",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL30",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL40",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL50",
        chip_num: 188,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL60",
        chip_num: 196,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL70",
        chip_num: 141,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVL80",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVM20",
        chip_num: 78,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60SVM40",
        chip_num: 86,
        chips_per_domain: 2,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VK10",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VK20",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VK30",
        chip_num: 215,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VK40",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VK6A",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VL10",
        chip_num: 111,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VL20",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VL30",
        chip_num: 123,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VL40",
        chip_num: 129,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M60VL50",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    // M61 Series
    MinerConfig {
        model: "M61S+VL30",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVK20",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVK30",
        chip_num: 235,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVL10",
        chip_num: 164,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVL20",
        chip_num: 172,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVL30",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVL60",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVL90",
        chip_num: 225,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M61SVM30",
        chip_num: 117,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VK10",
        chip_num: 180,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VK20",
        chip_num: 184,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VK30",
        chip_num: 188,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VK40",
        chip_num: 192,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VK60",
        chip_num: 188,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VL10",
        chip_num: 135,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VL30",
        chip_num: 141,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VL40",
        chip_num: 144,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VL50",
        chip_num: 147,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M61VL60",
        chip_num: 150,
        chips_per_domain: 3,
        board_num: 3,
    },
    // M62/M63 Series
    MinerConfig {
        model: "M62S+VK30",
        chip_num: 430,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M63S++VL20",
        chip_num: 380,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S++VL40",
        chip_num: 304,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S++VL50",
        chip_num: 340,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S++VL60",
        chip_num: 380,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S++VM20",
        chip_num: 198,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VK30",
        chip_num: 456,
        chips_per_domain: 6,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL10",
        chip_num: 304,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL20",
        chip_num: 340,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL30",
        chip_num: 370,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL50",
        chip_num: 272,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL60",
        chip_num: 304,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL70",
        chip_num: 240,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL80",
        chip_num: 256,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VL90",
        chip_num: 256,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VM30",
        chip_num: 136,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M63S+VM40",
        chip_num: 144,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK10",
        chip_num: 340,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK20",
        chip_num: 350,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK30",
        chip_num: 370,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK40",
        chip_num: 288,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK50",
        chip_num: 300,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK60",
        chip_num: 350,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK70",
        chip_num: 340,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK80",
        chip_num: 288,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVK90",
        chip_num: 304,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL10",
        chip_num: 228,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL20",
        chip_num: 216,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL30",
        chip_num: 272,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL50",
        chip_num: 288,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL60",
        chip_num: 288,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVL70",
        chip_num: 228,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63SVM30",
        chip_num: 132,
        chips_per_domain: 2,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VK10",
        chip_num: 256,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VK20",
        chip_num: 264,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VK30",
        chip_num: 272,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL10",
        chip_num: 174,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL20",
        chip_num: 204,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL30",
        chip_num: 216,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL40",
        chip_num: 180,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL60",
        chip_num: 216,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M63VL70",
        chip_num: 174,
        chips_per_domain: 3,
        board_num: 4,
    },
    // M64/M65/M66 Series
    MinerConfig {
        model: "M64S++VM30",
        chip_num: 96,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M64SVL10",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M64SVL20",
        chip_num: 120,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M64SVL30",
        chip_num: 152,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M64VL20",
        chip_num: 96,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M64VL30",
        chip_num: 114,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M64VL40",
        chip_num: 120,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M65S+VK30",
        chip_num: 456,
        chips_per_domain: 6,
        board_num: 4,
    },
    MinerConfig {
        model: "M65SVK20",
        chip_num: 350,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M65SVL60",
        chip_num: 288,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S++VL20",
        chip_num: 368,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M66S++VL40",
        chip_num: 288,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M66S++VL50",
        chip_num: 240,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S++VL60",
        chip_num: 250,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S++VM30",
        chip_num: 138,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VK30",
        chip_num: 440,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M66S+VL10",
        chip_num: 220,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL20",
        chip_num: 230,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL30",
        chip_num: 240,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL40",
        chip_num: 250,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL50",
        chip_num: 200,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL60",
        chip_num: 200,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66S+VL70",
        chip_num: 230,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVK20",
        chip_num: 368,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M66SVK30",
        chip_num: 384,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M66SVK40",
        chip_num: 240,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVK50",
        chip_num: 250,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVK60",
        chip_num: 250,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVK70",
        chip_num: 210,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVK80",
        chip_num: 220,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL10",
        chip_num: 168,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL20",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL30",
        chip_num: 192,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL40",
        chip_num: 200,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL50",
        chip_num: 210,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M66SVL80",
        chip_num: 160,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66VK20",
        chip_num: 184,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66VK30",
        chip_num: 192,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66VK60",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66VL20",
        chip_num: 160,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M66VL30",
        chip_num: 168,
        chips_per_domain: 4,
        board_num: 4,
    },
    // M67/M69/M70/M73/M76 Series
    MinerConfig {
        model: "M67SVK30",
        chip_num: 440,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M69S++VM30",
        chip_num: 228,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M69VK30",
        chip_num: 228,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M70SVM30",
        chip_num: 204,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M70VL30",
        chip_num: 255,
        chips_per_domain: 5,
        board_num: 3,
    },
    MinerConfig {
        model: "M70VM30",
        chip_num: 147,
        chips_per_domain: 3,
        board_num: 3,
    },
    MinerConfig {
        model: "M73SVM30",
        chip_num: 304,
        chips_per_domain: 4,
        board_num: 4,
    },
    MinerConfig {
        model: "M73VL30",
        chip_num: 380,
        chips_per_domain: 5,
        board_num: 4,
    },
    MinerConfig {
        model: "M73VM30",
        chip_num: 228,
        chips_per_domain: 3,
        board_num: 4,
    },
    MinerConfig {
        model: "M76SVM30",
        chip_num: 240,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M76VL30",
        chip_num: 384,
        chips_per_domain: 4,
        board_num: 3,
    },
    MinerConfig {
        model: "M76VM30",
        chip_num: 176,
        chips_per_domain: 4,
        board_num: 3,
    },
];
