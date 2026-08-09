# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.51M | 0.007 | 138.10M | 0.034 | 3.84× | 4.76× |
| 10,000 | 0.066 | 152.66M | 0.062 | 162.42M | 0.127 | 1.94× | 2.06× |
| 100,000 | 0.912 | 109.66M | 0.876 | 114.18M | 0.919 | 1.01× | 1.05× |
| 1,000,000 | 9.150 | 109.29M | 9.145 | 109.35M | 9.488 | 1.04× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.204 | 0.153 | 0.75× |
| 1 | 5 | 0.296 | 0.486 | 1.64× |
| 1 | 10 | 0.500 | 0.932 | 1.86× |
| 10 | 1 | 0.056 | 0.093 | 1.67× |
| 10 | 5 | 0.242 | 0.438 | 1.81× |
| 10 | 10 | 0.545 | 0.925 | 1.70× |
| 100 | 1 | 0.055 | 0.093 | 1.68× |
| 100 | 5 | 0.260 | 0.447 | 1.72× |
| 100 | 10 | 0.532 | 0.934 | 1.76× |
| 1,000 | 1 | 0.063 | 0.101 | 1.60× |
| 1,000 | 5 | 0.260 | 0.500 | 1.92× |
| 1,000 | 10 | 0.556 | 1.014 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
