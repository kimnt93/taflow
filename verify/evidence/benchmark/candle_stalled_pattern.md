# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.28M | 0.019 | 53.14M | 0.042 | 1.83× | 2.24× |
| 10,000 | 0.171 | 58.59M | 0.160 | 62.48M | 0.165 | 0.97× | 1.03× |
| 100,000 | 1.644 | 60.83M | 1.721 | 58.11M | 1.365 | 0.83× | 0.79× |
| 1,000,000 | 16.905 | 59.15M | 16.689 | 59.92M | 13.411 | 0.79× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.136 | 2.04× |
| 1 | 5 | 0.318 | 0.465 | 1.46× |
| 1 | 10 | 0.587 | 0.977 | 1.67× |
| 10 | 1 | 0.058 | 0.096 | 1.66× |
| 10 | 5 | 0.262 | 0.422 | 1.61× |
| 10 | 10 | 0.522 | 0.978 | 1.88× |
| 100 | 1 | 0.065 | 0.110 | 1.69× |
| 100 | 5 | 0.281 | 0.467 | 1.66× |
| 100 | 10 | 0.546 | 0.945 | 1.73× |
| 1,000 | 1 | 0.072 | 0.120 | 1.67× |
| 1,000 | 5 | 0.303 | 0.539 | 1.78× |
| 1,000 | 10 | 0.551 | 1.033 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
