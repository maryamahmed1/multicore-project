#!/usr/bin/env python3
import os
import subprocess
import csv

THREADS = [1, 2, 4, 8, 16, 32]
HEAT_N = [100, 500, 1000]
HEAT_ITERS = 100
PROG_PAGE_SIZE = [1000000, 5000000, 10000000]
CSV_FILENAME = "benchmark_results.csv"


def run_and_collect(cmd, cwd=None):
    results = []
    try:
        proc = subprocess.run(
            cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
        )
        if proc.returncode != 0:
            print(f"Error in {cmd}: {proc.stderr}")
        for line in proc.stdout.splitlines():
            if line.startswith("DATA:"):
                results.append(line.replace("DATA:", "").strip().split(","))
    except Exception as e:
        print(f"Failed to run {cmd}: {e}")
    return results


def main():
    print("Compiling OpenMP benchmarks...")
    os.makedirs("openMP/bin", exist_ok=True)
    subprocess.run(
        [
            "gcc",
            "-O3",
            "-fopenmp",
            "openMP/matrix_multiplier.c",
            "-o",
            "openMP/bin/mat_mul",
        ],
        check=True,
    )
    subprocess.run(
        ["gcc", "-O3", "-fopenmp", "openMP/heat_.c", "-o", "openMP/bin/heat"],
        check=True,
    )
    subprocess.run(
        ["gcc", "-O3", "-fopenmp", "openMP/programmability.c", "-o", "openMP/bin/prog"],
        check=True,
    )

    print("Compiling Rust benchmarks...")
    subprocess.run(["cargo", "build", "--release"], cwd="Rust", check=True)

    all_results = []

    print(f"Starting benchmarks for threads: {THREADS}")
    for t in THREADS:
        print(f"Testing thread count: {t}")

        all_results.extend(run_and_collect(["./openMP/bin/mat_mul", str(t)]))
        all_results.extend(
            run_and_collect(
                [
                    "cargo",
                    "run",
                    "--quiet",
                    "--release",
                    "--bin",
                    "matrix_multiplier",
                    "--",
                    str(t),
                ],
                cwd="Rust",
            )
        )

        for n in HEAT_N:
            all_results.extend(
                run_and_collect(["./openMP/bin/heat", str(n), str(HEAT_ITERS), str(t)])
            )

            all_results.extend(
                run_and_collect(
                    [
                        "cargo",
                        "run",
                        "--quiet",
                        "--release",
                        "--bin",
                        "heat",
                        "--",
                        str(n),
                        str(HEAT_ITERS),
                        str(t),
                    ],
                    cwd="Rust",
                )
            )

        for size in PROG_PAGE_SIZE:
            all_results.extend(
                run_and_collect(["./openMP/bin/prog", str(size), str(t)])
            )
            all_results.extend(
                run_and_collect(
                    [
                        "cargo",
                        "run",
                        "--quiet",
                        "--release",
                        "--bin",
                        "programmability",
                        "--",
                        str(size),
                        str(t),
                    ],
                    cwd="Rust",
                )
            )

    print(f"Writing results to {CSV_FILENAME}...")
    with open(CSV_FILENAME, mode="w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(
            ["Benchmark", "Language", "Problem_Size", "Threads", "Time_Seconds"]
        )
        writer.writerows(all_results)


if __name__ == "__main__":
    main()
