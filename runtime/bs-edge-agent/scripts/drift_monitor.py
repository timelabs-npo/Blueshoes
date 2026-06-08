#!/usr/bin/env python3
"""
Semantic Drift Monitor — Real-Time Telemetry Sensor for the Blueshoes Substrate

This script is the operational counterpart to the Rust drift engine.
It reads JSON-LD exports from the substrate, accepts streaming agent payloads,
and computes live KL divergence + Shannon entropy + Markov prediction.

Usage:
  # One-shot audit of a payload file against the substrate
  python3 drift_monitor.py audit --substrate substrate.jsonld --payload agent_output.json

  # Continuous watch mode (reads newline-delimited JSON from stdin)
  python3 drift_monitor.py watch --substrate substrate.jsonld

Exit codes:
  0 — clean (no drift detected)
  1 — drift alarm triggered
  2 — preemptive alarm triggered (predicted future drift)
  3 — both alarms triggered
"""

import json
import math
import sys
import argparse
from typing import Dict, List, Optional, Tuple


class DriftReport:
    """Full drift analysis result."""
    __slots__ = (
        'kl_divergence_bits', 'observed_entropy', 'baseline_entropy',
        'entropy_delta', 'drift_alarm', 'entropy_alarm',
        'predicted_kl_at_horizon', 'preemptive_alarm',
        'observed_distribution', 'ground_truth_distribution',
        'missing_concepts', 'hallucinated_concepts',
    )

    def __init__(self):
        self.kl_divergence_bits = 0.0
        self.observed_entropy = 0.0
        self.baseline_entropy = 0.0
        self.entropy_delta = 0.0
        self.drift_alarm = False
        self.entropy_alarm = False
        self.predicted_kl_at_horizon = 0.0
        self.preemptive_alarm = False
        self.observed_distribution = {}
        self.ground_truth_distribution = {}
        self.missing_concepts = 0
        self.hallucinated_concepts = 0

    def to_dict(self) -> dict:
        return {
            "kl_divergence_bits": round(self.kl_divergence_bits, 6),
            "observed_entropy": round(self.observed_entropy, 6),
            "baseline_entropy": round(self.baseline_entropy, 6),
            "entropy_delta": round(self.entropy_delta, 6),
            "drift_alarm": self.drift_alarm,
            "entropy_alarm": self.entropy_alarm,
            "predicted_kl_at_horizon": round(self.predicted_kl_at_horizon, 6),
            "preemptive_alarm": self.preemptive_alarm,
            "observed_distribution": self.observed_distribution,
            "ground_truth_distribution": self.ground_truth_distribution,
            "missing_concepts": self.missing_concepts,
            "hallucinated_concepts": self.hallucinated_concepts,
        }


class SemanticDriftMonitor:
    """
    Mathematical drift detector with three layers of defense:
      1. KL Divergence    — distribution shift detection
      2. Shannon Entropy   — structural chaos measurement
      3. Markov Predictor  — future drift trajectory forecasting
    """

    def __init__(
        self,
        kl_threshold: float = 0.35,
        entropy_threshold: float = 0.5,
        prediction_horizon: int = 5,
        smoothing_epsilon: float = 1e-6,
        drift_acceleration: float = 1.15,
    ):
        self.kl_threshold = kl_threshold
        self.entropy_threshold = entropy_threshold
        self.prediction_horizon = prediction_horizon
        self.smoothing_epsilon = smoothing_epsilon
        self.drift_acceleration = drift_acceleration
        self.kl_history: List[float] = []

    def analyze(
        self,
        ground_truth: Dict[str, float],
        observed_tokens: List[str],
    ) -> DriftReport:
        """Perform full drift analysis."""
        report = DriftReport()
        report.ground_truth_distribution = dict(ground_truth)

        total = len(observed_tokens)

        # Build observed distribution Q
        observed: Dict[str, float] = {}
        if total > 0:
            for token in observed_tokens:
                observed[token] = observed.get(token, 0.0) + 1.0
            for key in observed:
                observed[key] /= total
        report.observed_distribution = observed

        # Count missing and hallucinated concepts
        report.missing_concepts = sum(
            1 for k in ground_truth if k not in observed
        )
        report.hallucinated_concepts = sum(
            1 for k in observed if k not in ground_truth
        )

        # KL(P || Q)
        kl = self._kl_divergence(ground_truth, observed)
        report.kl_divergence_bits = kl

        # Shannon entropies
        report.baseline_entropy = self._shannon_entropy(ground_truth)
        report.observed_entropy = self._shannon_entropy(observed)
        report.entropy_delta = report.observed_entropy - report.baseline_entropy

        # Update history and predict
        self.kl_history.append(kl)
        predicted = self._markov_predict()
        report.predicted_kl_at_horizon = predicted

        # Alarms
        report.drift_alarm = kl > self.kl_threshold
        report.entropy_alarm = report.entropy_delta > self.entropy_threshold
        report.preemptive_alarm = predicted > self.kl_threshold

        return report

    def _kl_divergence(self, p: Dict[str, float], q: Dict[str, float]) -> float:
        """D_KL(P || Q) = Σ P(i) * log2(P(i) / Q(i))"""
        eps = self.smoothing_epsilon
        kl = 0.0
        for key, p_val in p.items():
            if p_val <= 0.0:
                continue
            q_val = max(q.get(key, eps), eps)
            kl += p_val * math.log2(p_val / q_val)
        return kl

    def _shannon_entropy(self, dist: Dict[str, float]) -> float:
        """H(X) = -Σ P(x_i) * log2(P(x_i))"""
        h = 0.0
        for p in dist.values():
            if p > 0.0:
                h -= p * math.log2(p)
        return h

    def _markov_predict(self) -> float:
        """Predict KL divergence N steps ahead using weighted Markov extrapolation."""
        n = len(self.kl_history)
        if n == 0:
            return 0.0
        if n == 1:
            step_delta = self.kl_history[0]
            predicted = self.kl_history[0]
            for step in range(self.prediction_horizon):
                predicted += step_delta * (self.drift_acceleration ** step)
            return predicted

        # Weighted average of recent deltas
        window = min(n, 10)
        weighted_delta = 0.0
        weight_sum = 0.0
        for i in range(n - window + 1, n):
            delta = self.kl_history[i] - self.kl_history[i - 1]
            recency_weight = 2.0 ** (i - n + 1)
            weighted_delta += delta * recency_weight
            weight_sum += recency_weight

        avg_delta = weighted_delta / weight_sum if weight_sum > 0 else 0.0

        # Project forward with acceleration
        predicted = self.kl_history[-1]
        for step in range(self.prediction_horizon):
            predicted += avg_delta * (self.drift_acceleration ** step)
        return max(0.0, predicted)

    def reset(self):
        """Reset prediction history after a forced context reset."""
        self.kl_history.clear()


def load_substrate_distribution(path: str) -> Dict[str, float]:
    """Load concept distribution from a substrate JSON-LD export."""
    with open(path) as f:
        data = json.load(f)
    dist = data.get("concept_distribution", {})
    if not dist:
        # Fallback: derive from graph nodes
        graph = data.get("@graph", [])
        if graph:
            total = len(graph)
            counts: Dict[str, float] = {}
            for node in graph:
                ntype = node.get("@type", "Unknown").replace("bs:", "")
                counts[ntype] = counts.get(ntype, 0.0) + 1.0
            dist = {k: v / total for k, v in counts.items()}
    return dist


def cmd_audit(args):
    """One-shot audit of a payload against the substrate."""
    ground_truth = load_substrate_distribution(args.substrate)
    with open(args.payload) as f:
        tokens = json.load(f)

    monitor = SemanticDriftMonitor(
        kl_threshold=args.threshold,
        entropy_threshold=args.entropy_threshold,
    )
    report = monitor.analyze(ground_truth, tokens)
    print(json.dumps(report.to_dict(), indent=2))

    exit_code = 0
    if report.drift_alarm:
        exit_code |= 1
    if report.preemptive_alarm:
        exit_code |= 2
    sys.exit(exit_code)


def cmd_watch(args):
    """Continuous watch mode: reads newline-delimited JSON arrays from stdin."""
    ground_truth = load_substrate_distribution(args.substrate)
    monitor = SemanticDriftMonitor(
        kl_threshold=args.threshold,
        entropy_threshold=args.entropy_threshold,
    )

    print(f"[drift_monitor] Watching stdin for agent payloads (threshold={args.threshold} bits)", file=sys.stderr)
    print(f"[drift_monitor] Ground truth: {ground_truth}", file=sys.stderr)

    for line_num, line in enumerate(sys.stdin, 1):
        line = line.strip()
        if not line:
            continue
        try:
            tokens = json.loads(line)
        except json.JSONDecodeError:
            print(f"[drift_monitor] WARNING: line {line_num} is not valid JSON, skipping", file=sys.stderr)
            continue

        report = monitor.analyze(ground_truth, tokens)
        output = {
            "line": line_num,
            **report.to_dict(),
        }
        print(json.dumps(output))
        sys.stdout.flush()

        if report.drift_alarm:
            print(f"[drift_monitor] ⚠ DRIFT ALARM at line {line_num}: KL={report.kl_divergence_bits:.4f} bits", file=sys.stderr)
        if report.preemptive_alarm:
            print(f"[drift_monitor] ⚠ PREEMPTIVE ALARM at line {line_num}: predicted KL={report.predicted_kl_at_horizon:.4f} bits", file=sys.stderr)


def cmd_selftest(_args):
    """Run internal self-test to validate the math."""
    gt = {"Person": 0.4, "Project": 0.4, "Metric": 0.2}
    monitor = SemanticDriftMonitor(kl_threshold=0.35)

    # Test 1: Perfect fidelity
    clean = ["Person", "Person", "Project", "Project", "Metric"]
    r = monitor.analyze(gt, clean)
    assert r.kl_divergence_bits < 0.01, f"FAIL: clean payload KL={r.kl_divergence_bits}"
    assert not r.drift_alarm, "FAIL: clean payload triggered drift alarm"
    print("✓ Test 1 PASSED: Perfect fidelity → zero drift")

    # Test 2: Skewed output
    monitor.reset()
    skewed = ["Person", "Person", "Person", "Person", "Project"]
    r = monitor.analyze(gt, skewed)
    assert r.kl_divergence_bits > 0.35, f"FAIL: skewed payload KL={r.kl_divergence_bits}"
    assert r.drift_alarm, "FAIL: skewed payload did not trigger drift alarm"
    assert r.missing_concepts == 1, f"FAIL: expected 1 missing concept, got {r.missing_concepts}"
    print(f"✓ Test 2 PASSED: Skewed output → KL={r.kl_divergence_bits:.4f} bits, alarm triggered")

    # Test 3: Hallucinated concepts
    monitor.reset()
    hallucinated = ["Person", "Project", "Metric", "UnknownGarbage", "Hallucinated"]
    r = monitor.analyze(gt, hallucinated)
    assert r.hallucinated_concepts == 2, f"FAIL: expected 2 hallucinated, got {r.hallucinated_concepts}"
    print(f"✓ Test 3 PASSED: Hallucinated concepts detected ({r.hallucinated_concepts})")

    # Test 4: Empty payload
    monitor.reset()
    r = monitor.analyze(gt, [])
    assert r.missing_concepts == 3, f"FAIL: expected 3 missing, got {r.missing_concepts}"
    print("✓ Test 4 PASSED: Empty payload → total drift")

    # Test 5: Markov prediction with accelerating drift
    monitor.reset()
    payloads = [
        ["Person", "Person", "Project", "Project", "Metric"],
        ["Person", "Person", "Person", "Project", "Metric"],
        ["Person", "Person", "Person", "Person", "Metric"],
        ["Person", "Person", "Person", "Person", "Person"],
    ]
    last_kl = 0.0
    for payload in payloads:
        r = monitor.analyze(gt, payload)
        assert r.kl_divergence_bits >= last_kl - 0.01, \
            f"FAIL: KL should be non-decreasing, got {r.kl_divergence_bits} < {last_kl}"
        last_kl = r.kl_divergence_bits

    assert r.preemptive_alarm, "FAIL: accelerating drift should trigger preemptive alarm"
    print(f"✓ Test 5 PASSED: Markov predictor detected accelerating drift (predicted KL={r.predicted_kl_at_horizon:.4f})")

    # Test 6: Shannon entropy sanity
    monitor_e = SemanticDriftMonitor()
    uniform = {"A": 0.25, "B": 0.25, "C": 0.25, "D": 0.25}
    h = monitor_e._shannon_entropy(uniform)
    assert abs(h - 2.0) < 1e-6, f"FAIL: uniform over 4 should have H=2.0, got {h}"
    degenerate = {"A": 1.0}
    h = monitor_e._shannon_entropy(degenerate)
    assert abs(h) < 1e-6, f"FAIL: degenerate should have H=0.0, got {h}"
    print("✓ Test 6 PASSED: Shannon entropy calculations correct")

    print("\n━━━ ALL 6 TESTS PASSED ━━━")


def main():
    parser = argparse.ArgumentParser(
        description="Semantic Drift Monitor — Telemetry sensor for the Blueshoes substrate"
    )
    sub = parser.add_subparsers(dest="command", required=True)

    # audit
    p_audit = sub.add_parser("audit", help="One-shot audit of a payload")
    p_audit.add_argument("--substrate", required=True, help="Path to substrate JSON-LD export")
    p_audit.add_argument("--payload", required=True, help="Path to JSON array of concept tokens")
    p_audit.add_argument("--threshold", type=float, default=0.35, help="KL alarm threshold (bits)")
    p_audit.add_argument("--entropy-threshold", type=float, default=0.5, help="Entropy delta threshold")

    # watch
    p_watch = sub.add_parser("watch", help="Continuous watch mode (reads from stdin)")
    p_watch.add_argument("--substrate", required=True, help="Path to substrate JSON-LD export")
    p_watch.add_argument("--threshold", type=float, default=0.35, help="KL alarm threshold (bits)")
    p_watch.add_argument("--entropy-threshold", type=float, default=0.5, help="Entropy delta threshold")

    # selftest
    sub.add_parser("selftest", help="Run internal math validation")

    args = parser.parse_args()
    {"audit": cmd_audit, "watch": cmd_watch, "selftest": cmd_selftest}[args.command](args)


if __name__ == "__main__":
    main()
