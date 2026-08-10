# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.96M | 0.018 | 56.97M | 0.036 | 1.67× | 2.03× |
| 10,000 | 0.163 | 61.47M | 0.157 | 63.73M | 0.126 | 0.77× | 0.80× |
| 100,000 | 1.599 | 62.55M | 1.647 | 60.70M | 1.048 | 0.66× | 0.64× |
| 1,000,000 | 16.352 | 61.15M | 15.896 | 62.91M | 9.842 | 0.60× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.129 | 0.98× |
| 1 | 5 | 0.301 | 0.455 | 1.51× |
| 1 | 10 | 0.549 | 0.960 | 1.75× |
| 10 | 1 | 0.066 | 0.088 | 1.35× |
| 10 | 5 | 0.250 | 0.446 | 1.78× |
| 10 | 10 | 0.538 | 0.989 | 1.84× |
| 100 | 1 | 0.068 | 0.107 | 1.58× |
| 100 | 5 | 0.315 | 0.482 | 1.53× |
| 100 | 10 | 0.588 | 0.914 | 1.56× |
| 1,000 | 1 | 0.083 | 0.101 | 1.21× |
| 1,000 | 5 | 0.258 | 0.543 | 2.10× |
| 1,000 | 10 | 0.628 | 1.024 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
