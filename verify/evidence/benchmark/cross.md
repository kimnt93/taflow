# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.39M | 0.008 | 123.66M | 0.021 | 2.30× | 2.62× |
| 10,000 | 0.053 | 189.19M | 0.048 | 207.69M | 0.044 | 0.84× | 0.92× |
| 100,000 | 0.524 | 190.80M | 0.473 | 211.51M | 0.286 | 0.54× | 0.60× |
| 1,000,000 | 5.174 | 193.28M | 4.630 | 215.98M | 6.390 | 1.23× | 1.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.095 | 0.93× |
| 1 | 5 | 0.360 | 0.346 | 0.96× |
| 1 | 10 | 0.471 | 0.696 | 1.48× |
| 10 | 1 | 0.053 | 0.070 | 1.32× |
| 10 | 5 | 0.228 | 0.343 | 1.50× |
| 10 | 10 | 0.486 | 0.727 | 1.50× |
| 100 | 1 | 0.052 | 0.066 | 1.26× |
| 100 | 5 | 0.220 | 0.351 | 1.59× |
| 100 | 10 | 0.482 | 0.721 | 1.50× |
| 1,000 | 1 | 0.056 | 0.076 | 1.34× |
| 1,000 | 5 | 0.226 | 0.664 | 2.94× |
| 1,000 | 10 | 0.477 | 1.212 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
