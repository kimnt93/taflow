# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.05M | 0.003 | 286.98M | 0.037 | 5.49× | 10.64× |
| 10,000 | 0.071 | 141.69M | 0.066 | 152.22M | 0.139 | 1.97× | 2.12× |
| 100,000 | 0.907 | 110.21M | 0.885 | 112.98M | 1.110 | 1.22× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.109 | 1.63× |
| 1 | 5 | 0.269 | 0.466 | 1.73× |
| 1 | 10 | 0.390 | 0.887 | 2.28× |
| 10 | 1 | 0.040 | 0.087 | 2.18× |
| 10 | 5 | 0.184 | 0.419 | 2.28× |
| 10 | 10 | 0.407 | 0.948 | 2.33× |
| 100 | 1 | 0.041 | 0.091 | 2.23× |
| 100 | 5 | 0.184 | 0.432 | 2.34× |
| 100 | 10 | 0.410 | 0.908 | 2.21× |
| 1,000 | 1 | 0.068 | 0.108 | 1.59× |
| 1,000 | 5 | 0.220 | 0.511 | 2.32× |
| 1,000 | 10 | 0.418 | 1.013 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
