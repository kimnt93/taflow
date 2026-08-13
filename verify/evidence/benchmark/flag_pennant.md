# FlagPennant benchmark (`FlagPennant` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.16M | 0.047 | 21.44M | 0.224 | 4.07× | 4.81× |
| 10,000 | 0.370 | 27.02M | 0.371 | 26.98M | 1.384 | 3.74× | 3.73× |
| 100,000 | 3.502 | 28.55M | 3.641 | 27.46M | 12.467 | 3.56× | 3.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.189 | 1.40× |
| 1 | 5 | 0.421 | 0.833 | 1.98× |
| 1 | 10 | 0.655 | 1.635 | 2.50× |
| 10 | 1 | 0.075 | 0.163 | 2.16× |
| 10 | 5 | 0.317 | 1.143 | 3.61× |
| 10 | 10 | 0.663 | 1.667 | 2.52× |
| 100 | 1 | 0.072 | 0.181 | 2.52× |
| 100 | 5 | 0.315 | 1.109 | 3.52× |
| 100 | 10 | 0.630 | 1.756 | 2.79× |
| 1,000 | 1 | 0.110 | 0.290 | 2.65× |
| 1,000 | 5 | 0.317 | 1.740 | 5.50× |
| 1,000 | 10 | 0.674 | 2.982 | 4.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
