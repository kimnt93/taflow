# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.15M | 0.010 | 99.74M | 0.040 | 2.91× | 4.02× |
| 10,000 | 0.161 | 62.17M | 0.157 | 63.53M | 0.176 | 1.10× | 1.12× |
| 100,000 | 1.745 | 57.31M | 1.710 | 58.48M | 1.454 | 0.83× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.108 | 1.30× |
| 1 | 5 | 0.192 | 0.441 | 2.30× |
| 1 | 10 | 0.384 | 0.928 | 2.42× |
| 10 | 1 | 0.046 | 0.099 | 2.18× |
| 10 | 5 | 0.197 | 0.455 | 2.31× |
| 10 | 10 | 0.413 | 0.931 | 2.25× |
| 100 | 1 | 0.059 | 0.091 | 1.55× |
| 100 | 5 | 0.189 | 0.456 | 2.41× |
| 100 | 10 | 0.423 | 0.970 | 2.29× |
| 1,000 | 1 | 0.058 | 0.106 | 1.82× |
| 1,000 | 5 | 0.185 | 0.526 | 2.84× |
| 1,000 | 10 | 0.436 | 1.156 | 2.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
