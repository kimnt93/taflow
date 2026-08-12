# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.74M | 0.016 | 63.58M | 0.035 | 2.01× | 2.26× |
| 10,000 | 0.138 | 72.42M | 0.137 | 73.10M | 0.140 | 1.01× | 1.02× |
| 100,000 | 1.405 | 71.16M | 1.448 | 69.08M | 1.200 | 0.85× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.129 | 1.31× |
| 1 | 5 | 0.307 | 0.500 | 1.63× |
| 1 | 10 | 0.607 | 1.086 | 1.79× |
| 10 | 1 | 0.069 | 0.103 | 1.50× |
| 10 | 5 | 0.295 | 0.664 | 2.25× |
| 10 | 10 | 1.078 | 1.068 | 0.99× |
| 100 | 1 | 0.077 | 0.131 | 1.70× |
| 100 | 5 | 0.345 | 0.653 | 1.89× |
| 100 | 10 | 0.695 | 1.407 | 2.02× |
| 1,000 | 1 | 0.109 | 0.163 | 1.49× |
| 1,000 | 5 | 0.423 | 0.764 | 1.81× |
| 1,000 | 10 | 0.947 | 1.184 | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
