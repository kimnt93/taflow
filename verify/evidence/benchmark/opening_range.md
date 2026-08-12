# OpeningRange benchmark (`anchored opening range` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.58M | 0.011 | 93.63M | 0.519 | 38.15× | 48.55× |
| 10,000 | 0.066 | 151.06M | 0.063 | 158.36M | 5.063 | 76.48× | 80.18× |
| 100,000 | 0.599 | 166.86M | 0.497 | 201.29M | 51.195 | 85.42× | 103.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.136 | 1.35× |
| 1 | 5 | 0.439 | 0.850 | 1.94× |
| 1 | 10 | 0.489 | 0.902 | 1.84× |
| 10 | 1 | 0.055 | 0.097 | 1.77× |
| 10 | 5 | 0.244 | 0.483 | 1.98× |
| 10 | 10 | 0.462 | 0.958 | 2.07× |
| 100 | 1 | 0.052 | 0.151 | 2.93× |
| 100 | 5 | 0.250 | 0.735 | 2.94× |
| 100 | 10 | 0.481 | 1.495 | 3.11× |
| 1,000 | 1 | 0.072 | 0.645 | 8.92× |
| 1,000 | 5 | 0.273 | 3.137 | 11.50× |
| 1,000 | 10 | 0.558 | 6.283 | 11.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
