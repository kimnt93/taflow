# Retracements benchmark (`causal swing retracements` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.54M | 0.042 | 23.93M | 4.525 | 92.93× | 108.27× |
| 10,000 | 0.408 | 24.50M | 0.394 | 25.40M | 46.741 | 114.52× | 118.70× |
| 100,000 | 3.976 | 25.15M | 3.792 | 26.37M | 473.383 | 119.05× | 124.82× |
| 1,000,000 | 40.880 | 24.46M | 39.017 | 25.63M | 4714.878 | 115.34× | 120.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.152 | 1.22× |
| 1 | 5 | 0.438 | 0.507 | 1.16× |
| 1 | 10 | 0.528 | 0.979 | 1.85× |
| 10 | 1 | 0.054 | 0.100 | 1.83× |
| 10 | 5 | 0.260 | 0.490 | 1.88× |
| 10 | 10 | 0.552 | 0.990 | 1.79× |
| 100 | 1 | 0.066 | 0.540 | 8.16× |
| 100 | 5 | 0.252 | 2.739 | 10.88× |
| 100 | 10 | 0.605 | 5.385 | 8.90× |
| 1,000 | 1 | 0.118 | 5.039 | 42.53× |
| 1,000 | 5 | 0.302 | 27.858 | 92.30× |
| 1,000 | 10 | 0.858 | 57.545 | 67.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
