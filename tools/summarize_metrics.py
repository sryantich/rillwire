#!/usr/bin/env python3
"""Summarize Measure-Client CSVs; no external packages or invented measurements."""
import csv
import json
import math
import statistics
import sys
from pathlib import Path


def percentile(values, fraction):
    return sorted(values)[max(0, math.ceil(len(values) * fraction) - 1)]


def summarize(path):
    with Path(path).open(encoding="utf-8-sig", newline="") as stream:
        rows = list(csv.DictReader(stream))
    if not rows:
        raise ValueError("No samples")
    present = [row for row in rows if int(row["process_count"]) > 0]
    if not present:
        raise ValueError("No running processes were sampled; no performance result")
    output = {"file": str(path), "samples": len(rows), "samples_with_process": len(present), "excluded_absent_samples": len(rows)-len(present)}
    for key, divisor in [("private_bytes", 1024**2), ("working_set_bytes_sum", 1024**2), ("CPU_one_core_percent", 1), ("CPU_machine_percent", 1)]:
        values = [float(row[key])/divisor for row in present if row[key].strip()]
        if values:
            output[key] = {"unit": "MiB" if divisor > 1 else "%", "median": statistics.median(values), "p95": percentile(values, .95), "p99": percentile(values, .99), "max": max(values)}
    return output


if __name__ == "__main__":
    if len(sys.argv) < 2:
        raise SystemExit("Usage: python tools/summarize_metrics.py captures/file.csv [...]")
    print(json.dumps([summarize(path) for path in sys.argv[1:]], indent=2))
