# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.20M | 0.006 | 177.46M | 0.037 | 5.62× | 6.55× |
| 10,000 | 0.036 | 281.08M | 0.040 | 252.33M | 0.063 | 1.76× | 1.58× |
| 100,000 | 0.331 | 301.80M | 0.294 | 339.60M | 0.334 | 1.01× | 1.13× |
| 1,000,000 | 3.865 | 258.72M | 3.106 | 321.99M | 3.054 | 0.79× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.123 | 1.42× |
| 1 | 5 | 0.338 | 0.546 | 1.61× |
| 1 | 10 | 0.514 | 1.048 | 2.04× |
| 10 | 1 | 0.057 | 0.105 | 1.84× |
| 10 | 5 | 0.266 | 0.507 | 1.90× |
| 10 | 10 | 0.539 | 1.007 | 1.87× |
| 100 | 1 | 0.050 | 0.102 | 2.01× |
| 100 | 5 | 0.264 | 0.486 | 1.84× |
| 100 | 10 | 0.509 | 1.001 | 1.97× |
| 1,000 | 1 | 0.056 | 0.102 | 1.84× |
| 1,000 | 5 | 0.246 | 0.496 | 2.02× |
| 1,000 | 10 | 0.543 | 1.010 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
