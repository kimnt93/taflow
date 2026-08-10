# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.63M | 0.017 | 59.99M | 0.031 | 1.61× | 1.87× |
| 10,000 | 0.128 | 77.97M | 0.127 | 78.88M | 0.080 | 0.62× | 0.63× |
| 100,000 | 1.229 | 81.36M | 1.183 | 84.50M | 0.551 | 0.45× | 0.47× |
| 1,000,000 | 12.778 | 78.26M | 12.146 | 82.33M | 5.647 | 0.44× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.103 | 1.53× |
| 1 | 5 | 0.440 | 0.456 | 1.04× |
| 1 | 10 | 0.565 | 0.867 | 1.53× |
| 10 | 1 | 0.053 | 0.089 | 1.68× |
| 10 | 5 | 0.249 | 0.421 | 1.69× |
| 10 | 10 | 0.551 | 0.885 | 1.61× |
| 100 | 1 | 0.058 | 0.088 | 1.50× |
| 100 | 5 | 0.254 | 0.421 | 1.66× |
| 100 | 10 | 0.524 | 1.212 | 2.31× |
| 1,000 | 1 | 0.093 | 0.121 | 1.30× |
| 1,000 | 5 | 0.299 | 0.481 | 1.61× |
| 1,000 | 10 | 0.633 | 1.018 | 1.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
