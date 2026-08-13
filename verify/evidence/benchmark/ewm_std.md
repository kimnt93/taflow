# ExponentiallyWeightedStandardDeviation benchmark (`ewm standard deviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.42M | 0.027 | 37.18M | 1.277 | 38.86× | 47.50× |
| 10,000 | 0.233 | 42.88M | 0.309 | 32.33M | 18.869 | 80.91× | 60.99× |
| 100,000 | 1.997 | 50.07M | 1.790 | 55.86M | 123.890 | 62.03× | 69.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.128 | 1.30× |
| 1 | 5 | 0.405 | 0.432 | 1.07× |
| 1 | 10 | 0.603 | 0.841 | 1.40× |
| 10 | 1 | 0.063 | 0.099 | 1.56× |
| 10 | 5 | 0.285 | 0.467 | 1.64× |
| 10 | 10 | 0.613 | 0.998 | 1.63× |
| 100 | 1 | 0.064 | 0.212 | 3.33× |
| 100 | 5 | 0.287 | 1.026 | 3.58× |
| 100 | 10 | 0.612 | 2.130 | 3.48× |
| 1,000 | 1 | 0.100 | 1.377 | 13.78× |
| 1,000 | 5 | 0.304 | 6.767 | 22.23× |
| 1,000 | 10 | 0.642 | 13.548 | 21.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
