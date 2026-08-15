# GoldenPocket benchmark (`GoldenPocket` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.81M | 0.013 | 75.97M | 0.502 | 33.04× | 38.14× |
| 10,000 | 0.140 | 71.20M | 0.144 | 69.37M | 4.035 | 28.73× | 27.99× |
| 100,000 | 1.367 | 73.13M | 1.343 | 74.43M | 42.855 | 31.34× | 31.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.224 | 3.66× |
| 1 | 5 | 0.273 | 0.829 | 3.04× |
| 1 | 10 | 0.454 | 1.957 | 4.31× |
| 10 | 1 | 0.045 | 0.179 | 3.95× |
| 10 | 5 | 0.207 | 0.836 | 4.05× |
| 10 | 10 | 0.480 | 1.927 | 4.01× |
| 100 | 1 | 0.050 | 0.203 | 4.06× |
| 100 | 5 | 0.200 | 1.033 | 5.15× |
| 100 | 10 | 0.458 | 2.289 | 5.00× |
| 1,000 | 1 | 0.065 | 0.819 | 12.61× |
| 1,000 | 5 | 0.243 | 10.106 | 41.65× |
| 1,000 | 10 | 0.484 | 6.385 | 13.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
