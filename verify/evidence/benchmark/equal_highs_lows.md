# EqualHighsLows benchmark (`causal equal pivot levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.254 | 3.94M | 0.247 | 4.05M | 4.436 | 17.48× | 17.98× |
| 10,000 | 2.333 | 4.29M | 2.328 | 4.30M | 44.185 | 18.94× | 18.98× |
| 100,000 | 23.161 | 4.32M | 22.897 | 4.37M | 445.211 | 19.22× | 19.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.194 | 0.205 | 1.06× |
| 1 | 5 | 0.462 | 0.707 | 1.53× |
| 1 | 10 | 0.682 | 1.481 | 2.17× |
| 10 | 1 | 0.080 | 0.173 | 2.18× |
| 10 | 5 | 0.343 | 0.821 | 2.40× |
| 10 | 10 | 0.740 | 1.701 | 2.30× |
| 100 | 1 | 0.103 | 0.546 | 5.31× |
| 100 | 5 | 0.352 | 2.751 | 7.82× |
| 100 | 10 | 0.709 | 5.474 | 7.72× |
| 1,000 | 1 | 0.336 | 4.680 | 13.92× |
| 1,000 | 5 | 0.604 | 23.704 | 39.26× |
| 1,000 | 10 | 1.055 | 57.124 | 54.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
