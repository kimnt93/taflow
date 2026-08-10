# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.47M | 0.025 | 39.25M | 0.959 | 31.14× | 37.64× |
| 10,000 | 0.224 | 44.55M | 0.194 | 51.63M | 5.563 | 24.78× | 28.72× |
| 100,000 | 2.030 | 49.27M | 2.003 | 49.93M | 59.180 | 29.16× | 29.55× |
| 1,000,000 | 21.067 | 47.47M | 18.466 | 54.15M | 637.209 | 30.25× | 34.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.307 | 3.33× |
| 1 | 5 | 0.383 | 1.238 | 3.23× |
| 1 | 10 | 0.595 | 2.536 | 4.26× |
| 10 | 1 | 0.061 | 0.284 | 4.65× |
| 10 | 5 | 0.276 | 1.386 | 5.03× |
| 10 | 10 | 0.600 | 2.651 | 4.42× |
| 100 | 1 | 0.066 | 0.300 | 4.52× |
| 100 | 5 | 0.304 | 1.755 | 5.78× |
| 100 | 10 | 0.718 | 3.330 | 4.64× |
| 1,000 | 1 | 0.083 | 1.000 | 12.02× |
| 1,000 | 5 | 0.318 | 4.558 | 14.32× |
| 1,000 | 10 | 0.680 | 8.846 | 13.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
