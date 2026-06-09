use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// MATHEMATICAL DRIFT DETECTION ENGINE
//
// This module treats semantic degradation as a measurable thermodynamic
// process. When a generative agent reads our stable substrate and emits
// work products, it introduces stochastic noise. We model this as entropy
// growth and detect it using information-theoretic divergence metrics.
//
// Three layers of defense:
//   1. KL Divergence              — detects distribution shift (hallucination onset)
//   2. Shannon Entropy             — detects structural chaos (random property injection)
//   3. Heuristic KL Extrapolator   — estimates future drift trajectory (preemptive kill)
// ═══════════════════════════════════════════════════════════════════════════

/// Complete drift analysis result. This is what the orchestration layer reads
/// to decide whether to trust an agent's output or force a context reset.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DriftReport {
    /// KL(P || Q) in bits. 0.0 = perfect fidelity. >threshold = hallucinating.
    pub kl_divergence_bits: f64,
    /// Shannon entropy of the observed distribution. Higher = more chaotic.
    pub observed_entropy: f64,
    /// Shannon entropy of the ground truth. Baseline for comparison.
    pub baseline_entropy: f64,
    /// Entropy delta (observed - baseline). Positive = degradation.
    pub entropy_delta: f64,
    /// Whether the KL divergence exceeds the configured alarm threshold.
    pub drift_alarm: bool,
    /// Whether the entropy delta exceeds the structural chaos threshold.
    pub entropy_alarm: bool,
    /// Estimated KL divergence N steps ahead (heuristic extrapolation, not a true Markov model).
    pub predicted_kl_at_horizon: f64,
    /// Whether the predicted future drift will breach the threshold.
    pub preemptive_alarm: bool,
    /// The observed concept distribution Q(x)
    pub observed_distribution: HashMap<String, f64>,
    /// The ground truth concept distribution P(x)
    pub ground_truth_distribution: HashMap<String, f64>,
    /// Number of concepts in ground truth that were completely absent from observed
    pub missing_concepts: usize,
    /// Number of concepts in observed that don't exist in ground truth (hallucinated types)
    pub hallucinated_concepts: usize,
}

/// Configuration for the drift detection thresholds.
#[derive(Debug, Clone)]
pub struct DriftConfig {
    /// KL divergence threshold in bits. Default: 0.35
    pub kl_threshold: f64,
    /// Entropy delta threshold. Default: 0.5
    pub entropy_threshold: f64,
    /// How many steps ahead to predict. Default: 5
    pub prediction_horizon: usize,
    /// Smoothing constant for unseen categories (Laplace). Default: 1e-6
    pub smoothing_epsilon: f64,
    /// Exponential scaling factor for heuristic KL extrapolation. Default: 1.15
    /// Values > 1.0 model accelerating drift (pessimistic, safer).
    pub drift_acceleration: f64,
}

impl Default for DriftConfig {
    fn default() -> Self {
        DriftConfig {
            kl_threshold: 0.35,
            entropy_threshold: 0.5,
            prediction_horizon: 5,
            smoothing_epsilon: 1e-6,
            drift_acceleration: 1.15,
        }
    }
}

pub struct DriftDetector {
    config: DriftConfig,
    /// Rolling history of KL divergence scores for heuristic extrapolation
    kl_history: Vec<f64>,
}

impl DriftDetector {
    pub fn new(config: DriftConfig) -> Self {
        DriftDetector {
            config,
            kl_history: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(DriftConfig::default())
    }

    /// Core analysis: given a ground-truth distribution P and an observed
    /// distribution Q (from agent output), compute the full drift report.
    pub fn analyze(
        &mut self,
        ground_truth: &HashMap<String, f64>,
        observed_tokens: &[String],
    ) -> DriftReport {
        let total = observed_tokens.len() as f64;

        // Build observed distribution Q
        let mut observed: HashMap<String, f64> = HashMap::new();
        if total > 0.0 {
            for token in observed_tokens {
                *observed.entry(token.clone()).or_insert(0.0) += 1.0;
            }
            for val in observed.values_mut() {
                *val /= total;
            }
        }

        // Count missing and hallucinated concepts
        let missing_concepts = ground_truth
            .keys()
            .filter(|k| !observed.contains_key(k.as_str()))
            .count();
        let hallucinated_concepts = observed
            .keys()
            .filter(|k| !ground_truth.contains_key(k.as_str()))
            .count();

        // Compute KL(P || Q)
        let kl = self.kl_divergence(ground_truth, &observed);

        // Compute Shannon entropies
        let baseline_entropy = self.shannon_entropy(ground_truth);
        let observed_entropy = self.shannon_entropy(&observed);
        let entropy_delta = observed_entropy - baseline_entropy;

        // Update history and predict
        self.kl_history.push(kl);
        let predicted_kl = self.extrapolate_kl();

        let drift_alarm = kl > self.config.kl_threshold;
        let entropy_alarm = entropy_delta > self.config.entropy_threshold;
        let preemptive_alarm = predicted_kl > self.config.kl_threshold;

        DriftReport {
            kl_divergence_bits: round6(kl),
            observed_entropy: round6(observed_entropy),
            baseline_entropy: round6(baseline_entropy),
            entropy_delta: round6(entropy_delta),
            drift_alarm,
            entropy_alarm,
            predicted_kl_at_horizon: round6(predicted_kl),
            preemptive_alarm,
            observed_distribution: observed.clone(),
            ground_truth_distribution: ground_truth.clone(),
            missing_concepts,
            hallucinated_concepts,
        }
    }

    /// D_KL(P || Q) = Σ P(i) * log2(P(i) / Q(i))
    ///
    /// If Q(i) = 0 for any i where P(i) > 0, the divergence is infinite.
    /// We use Laplace smoothing (epsilon) to prevent this while still
    /// producing a very high score — the agent dropped a core concept entirely.
    fn kl_divergence(&self, p: &HashMap<String, f64>, q: &HashMap<String, f64>) -> f64 {
        let eps = self.config.smoothing_epsilon;
        let mut kl = 0.0;
        for (key, &p_val) in p {
            if p_val <= 0.0 {
                continue;
            }
            let q_val = q.get(key).copied().unwrap_or(eps).max(eps);
            kl += p_val * (p_val / q_val).log2();
        }
        kl
    }

    /// H(X) = -Σ P(x_i) * log2(P(x_i))
    fn shannon_entropy(&self, dist: &HashMap<String, f64>) -> f64 {
        let mut h = 0.0;
        for &p in dist.values() {
            if p > 0.0 {
                h -= p * p.log2();
            }
        }
        h
    }

    /// Heuristic KL extrapolation: estimate KL divergence N steps ahead.
    ///
    /// This is a weighted-delta extrapolation with exponential acceleration,
    /// NOT a true Markov transition model. It assumes that drift compounds
    /// once an agent starts hallucinating. The drift_acceleration factor
    /// (default 1.15) models this pessimistic assumption.
    ///
    /// With only 1 data point, we assume linear growth from 0.
    /// With 2+, we compute the recency-weighted average step delta and project forward.
    fn extrapolate_kl(&self) -> f64 {
        let n = self.kl_history.len();
        if n == 0 {
            return 0.0;
        }
        if n == 1 {
            // Single observation: assume linear growth from zero
            let step_delta = self.kl_history[0];
            let mut predicted = self.kl_history[0];
            for step in 0..self.config.prediction_horizon {
                predicted += step_delta * self.config.drift_acceleration.powi(step as i32);
            }
            return predicted;
        }

        // Compute average step-over-step delta from recent history
        // Weight recent observations more heavily (exponential recency)
        let window = n.min(10); // Look at last 10 observations max
        let mut weighted_delta = 0.0;
        let mut weight_sum = 0.0;
        for i in (n - window + 1)..n {
            let delta = self.kl_history[i] - self.kl_history[i - 1];
            let recency_weight = 2.0_f64.powi((i as i32) - (n as i32) + 1); // e.g., 0.5, 1.0 for last two
            weighted_delta += delta * recency_weight;
            weight_sum += recency_weight;
        }
        let avg_delta = if weight_sum > 0.0 {
            weighted_delta / weight_sum
        } else {
            0.0
        };

        // Project forward with acceleration
        let current = self.kl_history[n - 1];
        let mut predicted = current;
        for step in 0..self.config.prediction_horizon {
            predicted += avg_delta * self.config.drift_acceleration.powi(step as i32);
        }
        predicted.max(0.0) // KL divergence can't go negative
    }

    /// Reset the prediction history. Call this after a forced context reset.
    pub fn reset_history(&mut self) {
        self.kl_history.clear();
    }

    /// Get the raw KL history for telemetry export.
    pub fn history(&self) -> &[f64] {
        &self.kl_history
    }
}

fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ground_truth() -> HashMap<String, f64> {
        let mut p = HashMap::new();
        p.insert("Person".to_string(), 0.4);
        p.insert("Project".to_string(), 0.4);
        p.insert("Metric".to_string(), 0.2);
        p
    }

    #[test]
    fn test_zero_drift_on_perfect_fidelity() {
        let mut detector = DriftDetector::with_defaults();
        let tokens = vec![
            "Person".into(),
            "Person".into(),
            "Project".into(),
            "Project".into(),
            "Metric".into(),
        ];
        let report = detector.analyze(&ground_truth(), &tokens);
        assert!(
            report.kl_divergence_bits < 0.01,
            "Perfect distribution should have near-zero KL"
        );
        assert!(!report.drift_alarm);
        assert!(!report.entropy_alarm);
    }

    #[test]
    fn test_drift_alarm_on_skewed_output() {
        let mut detector = DriftDetector::with_defaults();
        // Agent completely ignores Metric and skews to Person
        let tokens = vec![
            "Person".into(),
            "Person".into(),
            "Person".into(),
            "Person".into(),
            "Project".into(),
        ];
        let report = detector.analyze(&ground_truth(), &tokens);
        assert!(
            report.kl_divergence_bits > 0.35,
            "Skewed output should trigger drift"
        );
        assert!(report.drift_alarm);
        assert_eq!(report.missing_concepts, 1);
    }

    #[test]
    fn test_hallucinated_concept_detection() {
        let mut detector = DriftDetector::with_defaults();
        let tokens = vec![
            "Person".into(),
            "Project".into(),
            "Metric".into(),
            "UnknownGarbage".into(),
            "Hallucinated".into(),
        ];
        let report = detector.analyze(&ground_truth(), &tokens);
        assert_eq!(report.hallucinated_concepts, 2);
    }

    #[test]
    fn test_empty_payload_is_total_drift() {
        let mut detector = DriftDetector::with_defaults();
        let tokens: Vec<String> = vec![];
        let report = detector.analyze(&ground_truth(), &tokens);
        // Empty output means observed entropy = 0, all concepts missing
        assert_eq!(report.missing_concepts, 3);
    }

    #[test]
    fn test_heuristic_extrapolation_with_accelerating_drift() {
        let mut detector = DriftDetector::with_defaults();
        let gt = ground_truth();

        // Simulate progressively drifting payloads
        let payloads: Vec<Vec<String>> = vec![
            vec![
                "Person".into(),
                "Person".into(),
                "Project".into(),
                "Project".into(),
                "Metric".into(),
            ],
            vec![
                "Person".into(),
                "Person".into(),
                "Person".into(),
                "Project".into(),
                "Metric".into(),
            ],
            vec![
                "Person".into(),
                "Person".into(),
                "Person".into(),
                "Person".into(),
                "Metric".into(),
            ],
        ];

        let mut last_kl = 0.0;
        for payload in &payloads {
            let report = detector.analyze(&gt, payload);
            assert!(
                report.kl_divergence_bits >= last_kl - 0.01,
                "Drift should be non-decreasing with worsening input"
            );
            last_kl = report.kl_divergence_bits;
        }

        let final_report = detector.analyze(&gt, payloads.last().unwrap());
        // Prediction should be higher than current if drift is accelerating
        assert!(
            final_report.predicted_kl_at_horizon >= last_kl,
            "Predicted KL ({}) should be >= current KL ({})",
            final_report.predicted_kl_at_horizon,
            last_kl
        );
    }

    #[test]
    fn test_entropy_calculation() {
        let detector = DriftDetector::with_defaults();
        // Uniform distribution over 4 items: H = log2(4) = 2.0
        let mut uniform = HashMap::new();
        uniform.insert("A".to_string(), 0.25);
        uniform.insert("B".to_string(), 0.25);
        uniform.insert("C".to_string(), 0.25);
        uniform.insert("D".to_string(), 0.25);
        let h = detector.shannon_entropy(&uniform);
        assert!(
            (h - 2.0).abs() < 1e-6,
            "Uniform over 4 should have entropy 2.0, got {}",
            h
        );

        // Degenerate distribution: H = 0
        let mut degenerate = HashMap::new();
        degenerate.insert("A".to_string(), 1.0);
        let h = detector.shannon_entropy(&degenerate);
        assert!(
            h.abs() < 1e-6,
            "Degenerate should have entropy 0.0, got {}",
            h
        );
    }
}
