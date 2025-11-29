//! Spatial and cross-slot analysis of chip temperature grids
//!
//! Provides:
//! - Gradient: Local hotspot detection (chips hotter than neighbors)
//! - Outliers: Cross-slot comparison (chips hotter than same position on other boards)

use crate::models::Slot;

/// Analysis results for a single chip
#[derive(Debug, Clone, Copy, Default)]
pub struct ChipAnalysis {
    /// Local gradient: positive = hotter than neighbors (the bad case)
    /// Zero or negative values indicate chip is same or cooler than surroundings
    pub gradient: f32,
    /// Cross-slot z-score: how many std devs hotter than same position on other slots
    /// Positive = hotter than other boards at this position
    pub cross_slot_zscore: f32,
    /// Nonce deficit: percentage below slot average (0 = average, 100 = zero nonces)
    /// Higher = worse performance
    pub nonce_deficit: f32,
}

/// Analyze all slots together for cross-slot comparison
///
/// Returns a Vec of analysis results per slot, parallel to input slots.
/// Each inner Vec is parallel to that slot's chips.
pub fn analyze_all_slots(slots: &[Slot], chips_per_domain: usize) -> Vec<Vec<ChipAnalysis>> {
    if slots.is_empty() {
        return vec![];
    }

    // Find max chip count across all slots
    let max_chips = slots.iter().map(|s| s.chips.len()).max().unwrap_or(0);

    // Build cross-slot temperature matrix: temps_by_position[chip_idx] = [slot0_temp, slot1_temp, ...]
    let temps_by_position: Vec<Vec<i32>> = (0..max_chips)
        .map(|chip_idx| {
            slots
                .iter()
                .filter_map(|slot| slot.chips.get(chip_idx).map(|c| c.temp))
                .collect()
        })
        .collect();

    // Compute cross-slot stats for each position
    let cross_slot_stats: Vec<(f32, f32)> = temps_by_position
        .iter()
        .map(|temps| compute_mean_std(temps))
        .collect();

    // Analyze each slot
    slots
        .iter()
        .map(|slot| analyze_single_slot(slot, chips_per_domain, &cross_slot_stats))
        .collect()
}

/// Analyze a single slot with pre-computed cross-slot statistics
fn analyze_single_slot(
    slot: &Slot,
    chips_per_domain: usize,
    cross_slot_stats: &[(f32, f32)],
) -> Vec<ChipAnalysis> {
    let chips = &slot.chips;

    if chips.is_empty() || chips_per_domain == 0 {
        return vec![ChipAnalysis::default(); chips.len()];
    }

    let num_domains = chips.len().div_ceil(chips_per_domain);

    // Snake pattern section split (must match ui.rs logic)
    let remaining = num_domains.saturating_sub(1);
    let bottom_domains = 1 + remaining / 2;

    // Compute slot average nonce for performance comparison
    let slot_avg_nonce = compute_slot_avg_nonce(chips);

    chips
        .iter()
        .enumerate()
        .map(|(idx, chip)| {
            let domain = idx / chips_per_domain;
            let row = idx % chips_per_domain;

            // Determine if chip is in top or bottom section
            let is_top_section = domain >= bottom_domains;

            // Local gradient (only positive = hotter than upstream neighbors)
            let neighbors = get_upstream_neighbor_temps(
                chips,
                chips_per_domain,
                num_domains,
                domain,
                row,
                is_top_section,
            );
            let gradient = compute_hot_gradient(chip.temp, &neighbors);

            // Cross-slot comparison
            let cross_slot_zscore = if let Some(&(mean, std)) = cross_slot_stats.get(idx) {
                compute_hot_zscore(chip.temp, mean, std)
            } else {
                0.0
            };

            // Nonce performance deficit
            let nonce_deficit = compute_nonce_deficit(chip.nonce, slot_avg_nonce);

            ChipAnalysis {
                gradient,
                cross_slot_zscore,
                nonce_deficit,
            }
        })
        .collect()
}

/// Get temperature values of upstream neighbors (airflow-aware, snake-pattern-aware)
///
/// Physical layout with snake pattern:
/// ```
/// Top section:    [D30][D31]...[D58][D59]  ← D59 at RIGHT (intake)
/// Bottom section: [D29][D28]...[D1][D0]   ← D0 at RIGHT (intake)
/// ```
///
/// Airflow: right → left (intake on right side)
///
/// For BOTTOM section (D0 to D_bottom-1):
/// - Upstream (cooler) = lower domain (D-1)
///
/// For TOP section (D_bottom to D_max):
/// - Upstream (cooler) = HIGHER domain (D+1) because D_max is at intake!
fn get_upstream_neighbor_temps(
    chips: &[crate::models::Chip],
    cpd: usize,
    num_domains: usize,
    domain: usize,
    row: usize,
    is_top_section: bool,
) -> Vec<i32> {
    let mut neighbors = Vec::with_capacity(3);

    if is_top_section {
        // TOP SECTION: D_max is at intake (right), D_bottom is at exhaust (left)
        // Upstream = higher domain number (toward intake)
        if domain + 1 < num_domains {
            let idx = (domain + 1) * cpd + row;
            if idx < chips.len() {
                neighbors.push(chips[idx].temp);
            }
        }
        // NOTE: domain - 1 would be downstream (toward exhaust) - excluded
    } else {
        // BOTTOM SECTION: D0 is at intake (right), D_bottom-1 is at exhaust (left)
        // Upstream = lower domain number (toward intake)
        if domain > 0 {
            let idx = (domain - 1) * cpd + row;
            if idx < chips.len() {
                neighbors.push(chips[idx].temp);
            }
        }
        // NOTE: domain + 1 would be downstream (toward exhaust) - excluded

        // Special case: D0 has no upstream in bottom section, but D_max in top section
        // is at the SAME physical position (both at intake). Could compare, but skip for now.
    }

    // Up/down neighbors (row - 1, row + 1) = same airflow position in either section
    if row > 0 {
        let idx = domain * cpd + (row - 1);
        if idx < chips.len() {
            neighbors.push(chips[idx].temp);
        }
    }

    if row + 1 < cpd {
        let idx = domain * cpd + (row + 1);
        if idx < chips.len() {
            neighbors.push(chips[idx].temp);
        }
    }

    neighbors
}

/// Compute how much hotter this chip is than its neighbors
/// Returns 0 if chip is same temp or cooler (we only care about hot spots)
fn compute_hot_gradient(center: i32, neighbors: &[i32]) -> f32 {
    if neighbors.is_empty() {
        return 0.0;
    }

    let center_f = center as f32;
    let neighbor_avg: f32 =
        neighbors.iter().map(|&t| t as f32).sum::<f32>() / neighbors.len() as f32;

    // Only return positive values (hotter than neighbors)
    (center_f - neighbor_avg).max(0.0)
}

/// Compute mean and standard deviation
fn compute_mean_std(temps: &[i32]) -> (f32, f32) {
    if temps.is_empty() {
        return (0.0, 0.0);
    }

    let n = temps.len() as f32;
    let mean: f32 = temps.iter().map(|&t| t as f32).sum::<f32>() / n;

    if temps.len() == 1 {
        return (mean, 0.0);
    }

    let variance: f32 = temps
        .iter()
        .map(|&t| (t as f32 - mean).powi(2))
        .sum::<f32>()
        / n;
    (mean, variance.sqrt())
}

/// Compute z-score, but only for positive deviations (hotter than mean)
/// Returns 0 if chip is at or below the cross-slot mean
fn compute_hot_zscore(temp: i32, mean: f32, std: f32) -> f32 {
    let temp_f = temp as f32;
    let deviation = temp_f - mean;

    // Only care about chips hotter than the cross-slot average
    if deviation <= 0.0 {
        return 0.0;
    }

    // If std is very small, all slots are similar - any deviation is significant
    if std < 0.5 {
        // Small threshold to avoid division issues
        return deviation.min(3.0); // Cap at 3 for uniform temps
    }

    deviation / std
}

/// Compute average nonce count for a slot
fn compute_slot_avg_nonce(chips: &[crate::models::Chip]) -> f64 {
    if chips.is_empty() {
        return 0.0;
    }
    let total: i64 = chips.iter().map(|c| c.nonce).sum();
    total as f64 / chips.len() as f64
}

/// Compute nonce deficit as percentage below slot average
/// 0 = at or above average, 100 = zero nonces when average is non-zero
fn compute_nonce_deficit(chip_nonce: i64, slot_avg: f64) -> f32 {
    if slot_avg <= 0.0 {
        // No nonces on slot, can't compute deficit
        return 0.0;
    }

    let chip_nonce_f = chip_nonce as f64;
    if chip_nonce_f >= slot_avg {
        // At or above average - no deficit
        return 0.0;
    }

    // Deficit as percentage: (avg - chip) / avg * 100
    let deficit = (slot_avg - chip_nonce_f) / slot_avg * 100.0;
    deficit as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Chip;

    fn make_chip(id: i32, temp: i32) -> Chip {
        Chip {
            id,
            temp,
            ..Default::default()
        }
    }

    fn make_chip_with_nonce(id: i32, temp: i32, nonce: i64) -> Chip {
        Chip {
            id,
            temp,
            nonce,
            ..Default::default()
        }
    }

    fn make_slot(id: i32, temps: &[i32]) -> Slot {
        Slot {
            id,
            chips: temps
                .iter()
                .enumerate()
                .map(|(i, &t)| make_chip(i as i32, t))
                .collect(),
            ..Default::default()
        }
    }

    fn make_slot_with_nonces(id: i32, nonces: &[i64]) -> Slot {
        Slot {
            id,
            chips: nonces
                .iter()
                .enumerate()
                .map(|(i, &n)| make_chip_with_nonce(i as i32, 50, n))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn test_uniform_temps_no_gradient() {
        // 3x3 grid, all same temp
        let slots = vec![make_slot(0, &[50; 9])];
        let analysis = analyze_all_slots(&slots, 3);

        // All chips should have 0 gradient (no one is hotter)
        assert!(analysis[0].iter().all(|a| a.gradient < 0.1));
    }

    #[test]
    fn test_local_hotspot_detection() {
        // 3x3 grid with center hotspot
        let mut temps = [50; 9];
        temps[4] = 80; // Center is 30 degrees hotter

        let slots = vec![make_slot(0, &temps)];
        let analysis = analyze_all_slots(&slots, 3);

        // Center should have high gradient (local hotspot)
        assert!(analysis[0][4].gradient > 20.0);
        // Neighbors should have 0 or low gradient (they're cooler than center)
        assert!(analysis[0][1].gradient < 1.0); // Top neighbor
    }

    #[test]
    fn test_cold_spot_ignored() {
        // 3x3 grid with center cold spot
        let mut temps = [80; 9];
        temps[4] = 50; // Center is 30 degrees COOLER

        let slots = vec![make_slot(0, &temps)];
        let analysis = analyze_all_slots(&slots, 3);

        // Center should have 0 gradient (we don't flag cold spots)
        assert!(analysis[0][4].gradient < 0.1);
    }

    #[test]
    fn test_cross_slot_outlier() {
        // Three slots, chip 0 is hotter on slot 0
        let slots = vec![
            make_slot(0, &[90, 50, 50]), // Chip 0 is hot
            make_slot(1, &[50, 50, 50]),
            make_slot(2, &[50, 50, 50]),
        ];
        let analysis = analyze_all_slots(&slots, 3);

        // Chip 0 on slot 0 should be a cross-slot outlier
        assert!(analysis[0][0].cross_slot_zscore > 1.0);
        // Chip 0 on other slots should not be outliers
        assert!(analysis[1][0].cross_slot_zscore < 0.1);
        assert!(analysis[2][0].cross_slot_zscore < 0.1);
    }

    #[test]
    fn test_cross_slot_cooler_ignored() {
        // Three slots, chip 0 is COOLER on slot 0
        let slots = vec![
            make_slot(0, &[30, 50, 50]), // Chip 0 is cold
            make_slot(1, &[50, 50, 50]),
            make_slot(2, &[50, 50, 50]),
        ];
        let analysis = analyze_all_slots(&slots, 3);

        // Chip 0 on slot 0 should NOT be flagged (it's cooler, not a problem)
        assert!(analysis[0][0].cross_slot_zscore < 0.1);
    }

    #[test]
    fn test_airflow_bottom_section() {
        // 6 domains, 1 chip per domain
        // Section split: bottom_domains = 1 + (6-1)/2 = 1 + 2 = 3
        // Bottom section: D0, D1, D2 (D0 at right/intake)
        // Top section: D3, D4, D5 (D5 at right/intake)
        //
        // Physical layout:
        //   Top:    [D3][D4][D5]     ← D5 at intake
        //   Bottom: [D2][D1][D0]     ← D0 at intake
        //
        // Temps: D0=50, D1=60, D2=70 (bottom, normal gradient toward exhaust)
        let slots = vec![make_slot(0, &[50, 60, 70, 50, 50, 50])];
        let analysis = analyze_all_slots(&slots, 1);

        // Bottom section: upstream = lower domain (toward D0/intake)
        // D0: no upstream, gradient = 0
        assert!(analysis[0][0].gradient < 0.1);
        // D1: hotter than D0 (50) by 10°C
        assert!(analysis[0][1].gradient > 5.0);
        // D2: hotter than D1 (60) by 10°C
        assert!(analysis[0][2].gradient > 5.0);
    }

    #[test]
    fn test_airflow_top_section() {
        // 6 domains: bottom=D0,D1,D2; top=D3,D4,D5
        // Top section: D5 at right/intake, D3 at left/exhaust
        // For top section, upstream = HIGHER domain (toward D5)
        //
        // Temps: D3=80, D4=60, D5=50 (normal gradient: D3 hottest at exhaust)
        let slots = vec![make_slot(0, &[50, 50, 50, 80, 60, 50])];
        let analysis = analyze_all_slots(&slots, 1);

        // D3: upstream is D4 (60°C), D3 (80) is 20°C hotter
        assert!(
            analysis[0][3].gradient > 15.0,
            "D3 should have gradient, got {}",
            analysis[0][3].gradient
        );

        // D4: upstream is D5 (50°C), D4 (60) is 10°C hotter
        assert!(
            analysis[0][4].gradient > 5.0,
            "D4 should have gradient, got {}",
            analysis[0][4].gradient
        );

        // D5 has no upstream in top section (it's at intake)
        assert!(analysis[0][5].gradient < 0.1);
    }

    #[test]
    fn test_snake_boundary_chips() {
        // 6 domains: bottom=D0,D1,D2; top=D3,D4,D5
        // D2 is last of bottom section (at exhaust/left)
        // D3 is first of top section (also at exhaust/left)
        // They're at the SAME physical position!
        //
        // Temps: all 50 except D2=90 and D3=90
        let slots = vec![make_slot(0, &[50, 50, 90, 90, 50, 50])];
        let analysis = analyze_all_slots(&slots, 1);

        // D2 (bottom): upstream is D1 (50°C), D2 is 40°C hotter - flags!
        assert!(
            analysis[0][2].gradient > 30.0,
            "D2 should flag, got {}",
            analysis[0][2].gradient
        );

        // D3 (top): upstream is D4 (50°C), D3 is 40°C hotter - flags!
        assert!(
            analysis[0][3].gradient > 30.0,
            "D3 should flag, got {}",
            analysis[0][3].gradient
        );
    }

    #[test]
    fn test_nonce_uniform_no_deficit() {
        // All chips have same nonce count - no deficit
        let slots = vec![make_slot_with_nonces(0, &[1000, 1000, 1000])];
        let analysis = analyze_all_slots(&slots, 1);

        for (i, a) in analysis[0].iter().enumerate() {
            assert!(
                a.nonce_deficit < 0.1,
                "Chip {} should have no deficit, got {}",
                i,
                a.nonce_deficit
            );
        }
    }

    #[test]
    fn test_nonce_underperformer_detected() {
        // Chip 1 has half the nonces of others
        // Average = (1000 + 500 + 1000) / 3 = 833
        // Chip 1 deficit = (833 - 500) / 833 * 100 = 40%
        let slots = vec![make_slot_with_nonces(0, &[1000, 500, 1000])];
        let analysis = analyze_all_slots(&slots, 1);

        // Chip 0 and 2 are above average - no deficit
        assert!(analysis[0][0].nonce_deficit < 1.0);
        assert!(analysis[0][2].nonce_deficit < 1.0);

        // Chip 1 is underperforming - significant deficit
        assert!(
            analysis[0][1].nonce_deficit > 30.0,
            "Chip 1 should have ~40% deficit, got {}",
            analysis[0][1].nonce_deficit
        );
    }

    #[test]
    fn test_nonce_dead_chip_detected() {
        // Chip 1 has zero nonces - dead chip
        // Average = (1000 + 0 + 1000) / 3 = 666
        // Chip 1 deficit = (666 - 0) / 666 * 100 = 100%
        let slots = vec![make_slot_with_nonces(0, &[1000, 0, 1000])];
        let analysis = analyze_all_slots(&slots, 1);

        // Chip 1 should have 100% deficit (or close to it)
        assert!(
            analysis[0][1].nonce_deficit > 90.0,
            "Dead chip should have ~100% deficit, got {}",
            analysis[0][1].nonce_deficit
        );
    }

    #[test]
    fn test_nonce_overperformer_no_deficit() {
        // Chip 1 has MORE nonces than average - should not flag
        let slots = vec![make_slot_with_nonces(0, &[500, 1500, 500])];
        let analysis = analyze_all_slots(&slots, 1);

        // Chip 1 is above average - no deficit
        assert!(
            analysis[0][1].nonce_deficit < 0.1,
            "Overperformer should have no deficit, got {}",
            analysis[0][1].nonce_deficit
        );
    }
}
