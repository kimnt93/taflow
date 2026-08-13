# IntradayVolatilityProfile benchmark (`IntradayVolatilityProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.245 | 4.08M | 0.227 | 4.41M | 1.584 | 6.46× | 6.99× |
| 10,000 | 2.134 | 4.69M | 2.093 | 4.78M | 14.193 | 6.65× | 6.78× |
| 100,000 | 21.522 | 4.65M | 20.516 | 4.87M | 177.574 | 8.25× | 8.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.302 | 2.63× |
| 1 | 5 | 0.477 | 1.415 | 2.97× |
| 1 | 10 | 0.723 | 2.727 | 3.77× |
| 10 | 1 | 0.081 | 0.273 | 3.38× |
| 10 | 5 | 0.366 | 1.480 | 4.04× |
| 10 | 10 | 0.714 | 2.793 | 3.91× |
| 100 | 1 | 0.102 | 0.398 | 3.89× |
| 100 | 5 | 0.353 | 2.145 | 6.09× |
| 100 | 10 | 0.706 | 4.260 | 6.03× |
| 1,000 | 1 | 0.304 | 1.999 | 6.58× |
| 1,000 | 5 | 0.584 | 9.544 | 16.33× |
| 1,000 | 10 | 0.953 | 19.260 | 20.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
