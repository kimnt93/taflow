# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.85M | 0.015 | 67.90M | 0.044 | 2.56× | 3.00× |
| 10,000 | 0.172 | 58.15M | 0.174 | 57.44M | 0.184 | 1.07× | 1.06× |
| 100,000 | 1.810 | 55.25M | 1.787 | 55.96M | 1.574 | 0.87× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.123 | 1.02× |
| 1 | 5 | 0.320 | 0.463 | 1.45× |
| 1 | 10 | 0.392 | 0.904 | 2.31× |
| 10 | 1 | 0.051 | 0.094 | 1.86× |
| 10 | 5 | 0.188 | 0.434 | 2.31× |
| 10 | 10 | 0.406 | 0.931 | 2.29× |
| 100 | 1 | 0.042 | 0.088 | 2.07× |
| 100 | 5 | 0.202 | 0.441 | 2.18× |
| 100 | 10 | 0.404 | 0.989 | 2.45× |
| 1,000 | 1 | 0.064 | 0.100 | 1.56× |
| 1,000 | 5 | 0.192 | 0.508 | 2.65× |
| 1,000 | 10 | 0.412 | 1.072 | 2.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
