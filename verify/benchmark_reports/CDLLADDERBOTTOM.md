# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.49M | 0.007 | 140.27M | 0.032 | 3.70× | 4.53× |
| 10,000 | 0.059 | 169.24M | 0.056 | 179.11M | 0.082 | 1.39× | 1.47× |
| 100,000 | 0.595 | 167.94M | 0.579 | 172.75M | 0.628 | 1.06× | 1.09× |
| 1,000,000 | 6.708 | 149.08M | 6.087 | 164.29M | 6.038 | 0.90× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.102 | 1.25× |
| 1 | 5 | 0.282 | 0.478 | 1.70× |
| 1 | 10 | 0.525 | 0.957 | 1.82× |
| 10 | 1 | 0.054 | 0.095 | 1.76× |
| 10 | 5 | 0.259 | 0.455 | 1.76× |
| 10 | 10 | 0.517 | 0.934 | 1.81× |
| 100 | 1 | 0.054 | 0.094 | 1.74× |
| 100 | 5 | 0.275 | 0.460 | 1.67× |
| 100 | 10 | 0.550 | 1.008 | 1.83× |
| 1,000 | 1 | 0.062 | 0.100 | 1.62× |
| 1,000 | 5 | 0.301 | 0.578 | 1.92× |
| 1,000 | 10 | 0.632 | 1.121 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
