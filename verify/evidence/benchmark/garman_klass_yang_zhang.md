# GarmanKlassYangZhang benchmark (`annualized Garman-Klass-Yang-Zhang volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.35M | 0.029 | 35.08M | 0.119 | 3.86× | 4.18× |
| 10,000 | 0.227 | 44.03M | 0.222 | 44.97M | 0.459 | 2.02× | 2.07× |
| 100,000 | 2.136 | 46.82M | 2.114 | 47.30M | 3.773 | 1.77× | 1.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.135 | 1.43× |
| 1 | 5 | 0.323 | 0.800 | 2.48× |
| 1 | 10 | 0.535 | 1.211 | 2.27× |
| 10 | 1 | 0.055 | 0.125 | 2.26× |
| 10 | 5 | 0.255 | 0.604 | 2.37× |
| 10 | 10 | 0.553 | 1.234 | 2.23× |
| 100 | 1 | 0.063 | 0.167 | 2.65× |
| 100 | 5 | 0.267 | 0.837 | 3.13× |
| 100 | 10 | 0.569 | 1.612 | 2.83× |
| 1,000 | 1 | 0.082 | 0.207 | 2.53× |
| 1,000 | 5 | 0.278 | 1.089 | 3.91× |
| 1,000 | 10 | 0.601 | 2.401 | 3.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
