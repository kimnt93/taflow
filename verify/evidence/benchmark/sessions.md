# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.39M | 0.018 | 56.39M | 94.842 | 4873.85× | 5348.41× |
| 10,000 | 0.086 | 115.80M | 0.075 | 133.24M | 925.281 | 10714.61× | 12328.66× |
| 100,000 | 0.762 | 131.31M | 0.712 | 140.37M | 9294.050 | 12203.97× | 13046.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 1.918 | 23.98× |
| 1 | 5 | 0.264 | 8.622 | 32.70× |
| 1 | 10 | 0.554 | 18.362 | 33.15× |
| 10 | 1 | 0.057 | 2.551 | 44.49× |
| 10 | 5 | 0.263 | 13.675 | 51.90× |
| 10 | 10 | 0.559 | 29.226 | 52.26× |
| 100 | 1 | 0.071 | 11.538 | 163.48× |
| 100 | 5 | 0.424 | 56.963 | 134.28× |
| 100 | 10 | 0.670 | 118.134 | 176.35× |
| 1,000 | 1 | 0.099 | 96.225 | 971.26× |
| 1,000 | 5 | 0.525 | 538.574 | 1026.25× |
| 1,000 | 10 | 0.617 | 1238.481 | 2007.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
