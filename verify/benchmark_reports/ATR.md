# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.40M | 0.009 | 108.06M | 0.041 | 3.72× | 4.44× |
| 10,000 | 0.061 | 164.04M | 0.057 | 174.04M | 0.097 | 1.59× | 1.69× |
| 100,000 | 0.564 | 177.28M | 0.553 | 180.80M | 0.681 | 1.21× | 1.23× |
| 1,000,000 | 6.203 | 161.22M | 5.655 | 176.84M | 6.628 | 1.07× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.140 | 1.57× |
| 1 | 5 | 0.341 | 0.565 | 1.66× |
| 1 | 10 | 0.542 | 1.041 | 1.92× |
| 10 | 1 | 0.050 | 0.099 | 1.97× |
| 10 | 5 | 0.274 | 0.511 | 1.87× |
| 10 | 10 | 0.556 | 1.070 | 1.92× |
| 100 | 1 | 0.054 | 0.099 | 1.83× |
| 100 | 5 | 0.251 | 0.500 | 1.99× |
| 100 | 10 | 0.550 | 1.073 | 1.95× |
| 1,000 | 1 | 0.057 | 0.103 | 1.81× |
| 1,000 | 5 | 0.263 | 0.510 | 1.93× |
| 1,000 | 10 | 0.593 | 1.175 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
