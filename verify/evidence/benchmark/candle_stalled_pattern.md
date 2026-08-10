# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.72M | 0.022 | 45.68M | 0.049 | 1.87× | 2.26× |
| 10,000 | 0.194 | 51.65M | 0.209 | 47.78M | 0.192 | 0.99× | 0.92× |
| 100,000 | 2.567 | 38.95M | 2.101 | 47.59M | 1.757 | 0.68× | 0.84× |
| 1,000,000 | 20.671 | 48.38M | 20.374 | 49.08M | 17.057 | 0.83× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.138 | 1.29× |
| 1 | 5 | 0.336 | 0.563 | 1.68× |
| 1 | 10 | 0.718 | 1.256 | 1.75× |
| 10 | 1 | 0.064 | 0.105 | 1.66× |
| 10 | 5 | 0.383 | 0.599 | 1.56× |
| 10 | 10 | 0.801 | 1.220 | 1.52× |
| 100 | 1 | 0.079 | 0.111 | 1.41× |
| 100 | 5 | 0.382 | 0.683 | 1.79× |
| 100 | 10 | 0.760 | 1.220 | 1.61× |
| 1,000 | 1 | 0.089 | 0.134 | 1.51× |
| 1,000 | 5 | 0.489 | 0.807 | 1.65× |
| 1,000 | 10 | 0.897 | 1.379 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
