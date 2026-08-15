# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.65M | 0.007 | 153.77M | 0.039 | 4.80× | 5.92× |
| 10,000 | 0.059 | 168.80M | 0.057 | 175.00M | 0.087 | 1.47× | 1.52× |
| 100,000 | 0.571 | 175.22M | 0.540 | 185.20M | 0.561 | 0.98× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.111 | 1.27× |
| 1 | 5 | 0.399 | 0.523 | 1.31× |
| 1 | 10 | 0.445 | 0.981 | 2.20× |
| 10 | 1 | 0.042 | 0.093 | 2.22× |
| 10 | 5 | 0.185 | 0.481 | 2.61× |
| 10 | 10 | 0.419 | 1.021 | 2.44× |
| 100 | 1 | 0.045 | 0.091 | 2.03× |
| 100 | 5 | 0.202 | 0.462 | 2.29× |
| 100 | 10 | 0.387 | 0.984 | 2.54× |
| 1,000 | 1 | 0.054 | 0.110 | 2.01× |
| 1,000 | 5 | 0.210 | 0.472 | 2.25× |
| 1,000 | 10 | 0.406 | 1.019 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
