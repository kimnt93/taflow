# IntradayVolatilityProfile benchmark (`IntradayVolatilityProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.98M | 0.064 | 15.64M | 1.611 | 22.52× | 25.20× |
| 10,000 | 0.585 | 17.10M | 0.539 | 18.56M | 15.088 | 25.81× | 28.01× |
| 100,000 | 6.328 | 15.80M | 5.050 | 19.80M | 187.056 | 29.56× | 37.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.349 | 2.81× |
| 1 | 5 | 0.326 | 1.511 | 4.63× |
| 1 | 10 | 0.614 | 3.045 | 4.96× |
| 10 | 1 | 0.081 | 0.300 | 3.71× |
| 10 | 5 | 0.305 | 1.659 | 5.45× |
| 10 | 10 | 0.625 | 3.209 | 5.13× |
| 100 | 1 | 0.068 | 0.436 | 6.42× |
| 100 | 5 | 0.287 | 2.316 | 8.06× |
| 100 | 10 | 0.672 | 4.398 | 6.54× |
| 1,000 | 1 | 0.120 | 2.062 | 17.25× |
| 1,000 | 5 | 0.306 | 9.603 | 31.33× |
| 1,000 | 10 | 0.703 | 19.853 | 28.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
