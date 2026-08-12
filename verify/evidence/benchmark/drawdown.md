# Drawdown benchmark (`drawdown from cumulative maximum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.32M | 0.007 | 145.86M | 0.024 | 3.19× | 3.46× |
| 10,000 | 0.048 | 208.68M | 0.045 | 221.64M | 0.064 | 1.34× | 1.42× |
| 100,000 | 0.441 | 226.59M | 0.392 | 254.95M | 0.437 | 0.99× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.142 | 0.85× |
| 1 | 5 | 0.259 | 0.365 | 1.41× |
| 1 | 10 | 0.483 | 0.701 | 1.45× |
| 10 | 1 | 0.047 | 0.069 | 1.47× |
| 10 | 5 | 0.249 | 0.368 | 1.48× |
| 10 | 10 | 0.501 | 0.729 | 1.46× |
| 100 | 1 | 0.049 | 0.071 | 1.46× |
| 100 | 5 | 0.222 | 0.347 | 1.56× |
| 100 | 10 | 0.496 | 0.770 | 1.55× |
| 1,000 | 1 | 0.054 | 0.078 | 1.44× |
| 1,000 | 5 | 0.242 | 0.421 | 1.74× |
| 1,000 | 10 | 0.512 | 0.934 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
