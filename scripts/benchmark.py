#!/usr/bin/env python3
"""Benchmark bkg-finder C and Rust implementations.

Measures wall time, user time, system time, and max RSS using Python's
resource module instead of /usr/bin/time (not available on this system).
"""

import subprocess
import sys
import os
import time
import resource

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

INPUT_COUNTS = [10, 100, 1000, 10000, 1_000_000]

HEADERS = ["program", "inputs", "wall_ms", "user_ms", "sys_ms",
           "max_rss_kb", "exit_code", "ok", "fail"]


def build_rust() -> str:
    print("[BUILD] Rust release + bench_rust ...")
    subprocess.run(
        ["cargo", "build", "--release", "--bin", "bench_rust"],
        cwd=PROJECT_ROOT, check=True, capture_output=True, text=True)
    return os.path.join(PROJECT_ROOT, "target", "release", "bench_rust")


def build_c() -> str:
    print("[BUILD] C benchmark ...")
    out = os.path.join(PROJECT_ROOT, "target", "bench_c")
    subprocess.run(
        ["gcc", "-O3", "-lm",
         "-o", out,
         os.path.join(PROJECT_ROOT, "bench", "bench_c.c")],
        cwd=PROJECT_ROOT, check=True, capture_output=True, text=True)
    return out


def run_bench(exe: str, count: int) -> dict:
    """Run exe with count argument, collect per-run resource usage."""
    ru_before = resource.getrusage(resource.RUSAGE_CHILDREN)

    t0_wall = time.perf_counter()
    proc = subprocess.run(
        [exe, str(count)],
        capture_output=True, text=True, timeout=600,
        preexec_fn=os.setsid)
    t1_wall = time.perf_counter()

    ru_after = resource.getrusage(resource.RUSAGE_CHILDREN)

    wall_ms = int((t1_wall - t0_wall) * 1000)
    user_ms = int((ru_after.ru_utime - ru_before.ru_utime) * 1000)
    sys_ms = int((ru_after.ru_stime - ru_before.ru_stime) * 1000)
    max_rss_kb = max(ru_before.ru_maxrss, ru_after.ru_maxrss)

    stdout = proc.stdout.strip()
    parts = dict(item.split("=") for item in stdout.split())
    ok = int(parts.get("ok", "?"))
    fail = int(parts.get("fail", "?"))

    return {
        "wall_ms": wall_ms,
        "user_ms": user_ms,
        "sys_ms": sys_ms,
        "max_rss_kb": max_rss_kb,
        "exit_code": proc.returncode,
        "ok": ok,
        "fail": fail,
    }


def fmt_row(cols, widths):
    return " | ".join(str(c).ljust(w) for c, w in zip(cols, widths))


def main():
    rust_bin = build_rust()
    c_bin = build_c()

    results = []
    for count in INPUT_COUNTS:
        print(f"\n[RUN] count={count}")
        for label, exe in [("c", c_bin), ("rust", rust_bin)]:
            print(f"       {label} ...", end=" ", flush=True)
            data = run_bench(exe, count)
            data["program"] = label
            data["inputs"] = count
            results.append(data)
            print(f"ok={data['ok']} fail={data['fail']} "
                  f"wall={data['wall_ms']}ms rss={data['max_rss_kb']}KB")

    print("\n## Results\n")
    widths = [8, 8, 10, 10, 10, 12, 10, 6, 6]
    print(fmt_row(HEADERS, widths))
    print(" | ".join("-" * w for w in widths))
    for r in results:
        row = [r[h] for h in HEADERS]
        print(fmt_row(row, widths))


if __name__ == "__main__":
    main()
