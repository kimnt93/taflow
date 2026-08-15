# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.01M | 0.011 | 87.87M | 0.033 | 2.27× | 2.94× |
| 10,000 | 0.138 | 72.69M | 0.131 | 76.57M | 0.082 | 0.60× | 0.63× |
| 100,000 | 1.357 | 73.70M | 1.346 | 74.30M | 0.602 | 0.44× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.110 | 1.43× |
| 1 | 5 | 0.252 | 0.456 | 1.81× |
| 1 | 10 | 0.379 | 0.953 | 2.51× |
| 10 | 1 | 0.046 | 0.095 | 2.08× |
| 10 | 5 | 0.184 | 0.431 | 2.34× |
| 10 | 10 | 0.400 | 0.925 | 2.31× |
| 100 | 1 | 0.046 | 0.092 | 2.01× |
| 100 | 5 | 0.196 | 0.470 | 2.40× |
| 100 | 10 | 0.420 | 0.931 | 2.22× |
| 1,000 | 1 | 0.059 | 0.093 | 1.56× |
| 1,000 | 5 | 0.191 | 0.444 | 2.33× |
| 1,000 | 10 | 0.418 | 1.039 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
