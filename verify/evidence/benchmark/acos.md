# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.17M | 0.012 | 86.92M | 0.057 | 2.75× | 4.96× |
| 10,000 | 0.113 | 88.67M | 0.082 | 121.84M | 0.151 | 1.34× | 1.84× |
| 100,000 | 1.422 | 70.31M | 1.315 | 76.06M | 1.196 | 0.84× | 0.91× |
| 1,000,000 | 8.730 | 114.55M | 7.962 | 125.59M | 13.211 | 1.51× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.162 | 1.69× |
| 1 | 5 | 0.464 | 0.595 | 1.28× |
| 1 | 10 | 0.661 | 1.417 | 2.14× |
| 10 | 1 | 0.067 | 0.128 | 1.91× |
| 10 | 5 | 0.351 | 0.648 | 1.84× |
| 10 | 10 | 0.567 | 1.327 | 2.34× |
| 100 | 1 | 0.076 | 0.141 | 1.86× |
| 100 | 5 | 0.343 | 0.586 | 1.71× |
| 100 | 10 | 1.339 | 1.306 | 0.98× |
| 1,000 | 1 | 0.073 | 0.126 | 1.72× |
| 1,000 | 5 | 0.409 | 0.933 | 2.28× |
| 1,000 | 10 | 0.735 | 1.615 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
