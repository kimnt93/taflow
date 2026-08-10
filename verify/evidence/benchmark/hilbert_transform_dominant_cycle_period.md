# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.16M | 0.046 | 21.89M | 0.069 | 1.53× | 1.51× |
| 10,000 | 0.435 | 23.01M | 0.426 | 23.46M | 0.438 | 1.01× | 1.03× |
| 100,000 | 4.271 | 23.41M | 4.310 | 23.20M | 4.141 | 0.97× | 0.96× |
| 1,000,000 | 44.045 | 22.70M | 43.690 | 22.89M | 42.441 | 0.96× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.119 | 1.04× |
| 1 | 5 | 0.409 | 0.460 | 1.12× |
| 1 | 10 | 0.469 | 0.869 | 1.85× |
| 10 | 1 | 0.046 | 0.083 | 1.83× |
| 10 | 5 | 0.221 | 0.429 | 1.95× |
| 10 | 10 | 0.443 | 0.889 | 2.01× |
| 100 | 1 | 0.053 | 0.092 | 1.73× |
| 100 | 5 | 0.227 | 0.431 | 1.90× |
| 100 | 10 | 0.466 | 0.920 | 1.97× |
| 1,000 | 1 | 0.103 | 0.133 | 1.30× |
| 1,000 | 5 | 0.225 | 0.651 | 2.90× |
| 1,000 | 10 | 0.495 | 1.352 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
