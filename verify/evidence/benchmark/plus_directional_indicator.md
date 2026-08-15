# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.58M | 0.007 | 149.95M | 0.039 | 4.34× | 5.88× |
| 10,000 | 0.061 | 164.34M | 0.057 | 174.36M | 0.101 | 1.65× | 1.75× |
| 100,000 | 0.598 | 167.25M | 0.542 | 184.38M | 0.690 | 1.15× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.103 | 0.90× |
| 1 | 5 | 0.277 | 0.524 | 1.90× |
| 1 | 10 | 0.422 | 0.942 | 2.23× |
| 10 | 1 | 0.046 | 0.089 | 1.95× |
| 10 | 5 | 0.177 | 0.436 | 2.46× |
| 10 | 10 | 0.403 | 1.043 | 2.59× |
| 100 | 1 | 0.042 | 0.101 | 2.39× |
| 100 | 5 | 0.189 | 0.469 | 2.48× |
| 100 | 10 | 0.388 | 0.995 | 2.57× |
| 1,000 | 1 | 0.054 | 0.106 | 1.96× |
| 1,000 | 5 | 0.221 | 0.494 | 2.24× |
| 1,000 | 10 | 0.399 | 1.014 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
