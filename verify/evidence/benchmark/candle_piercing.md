# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.57M | 0.089 | 11.23M | 0.031 | 0.30× | 0.35× |
| 10,000 | 0.821 | 12.19M | 0.810 | 12.35M | 0.111 | 0.14× | 0.14× |
| 100,000 | 8.512 | 11.75M | 7.974 | 12.54M | 0.928 | 0.11× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.109 | 0.92× |
| 1 | 5 | 0.395 | 0.452 | 1.14× |
| 1 | 10 | 0.625 | 0.906 | 1.45× |
| 10 | 1 | 0.070 | 0.093 | 1.32× |
| 10 | 5 | 0.297 | 0.424 | 1.43× |
| 10 | 10 | 0.628 | 0.937 | 1.49× |
| 100 | 1 | 0.087 | 0.090 | 1.04× |
| 100 | 5 | 0.340 | 0.489 | 1.44× |
| 100 | 10 | 0.680 | 0.929 | 1.37× |
| 1,000 | 1 | 0.152 | 0.096 | 0.63× |
| 1,000 | 5 | 0.352 | 0.483 | 1.37× |
| 1,000 | 10 | 0.684 | 1.026 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
