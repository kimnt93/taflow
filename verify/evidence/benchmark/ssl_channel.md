# SmoothedTrendChannel benchmark (`smoothed trend channel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.72M | 0.010 | 95.90M | 0.573 | 43.97× | 54.97× |
| 10,000 | 0.123 | 81.05M | 0.115 | 87.29M | 5.227 | 42.37× | 45.63× |
| 100,000 | 1.199 | 83.39M | 1.197 | 83.58M | 50.540 | 42.14× | 42.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.050 | 0.203 | 4.06× |
| 1 | 5 | 0.236 | 0.543 | 2.30× |
| 1 | 10 | 0.386 | 1.098 | 2.85× |
| 10 | 1 | 0.044 | 0.187 | 4.23× |
| 10 | 5 | 0.194 | 0.850 | 4.37× |
| 10 | 10 | 0.419 | 1.791 | 4.27× |
| 100 | 1 | 0.050 | 0.220 | 4.42× |
| 100 | 5 | 0.213 | 1.091 | 5.13× |
| 100 | 10 | 0.422 | 2.377 | 5.63× |
| 1,000 | 1 | 0.057 | 0.681 | 11.95× |
| 1,000 | 5 | 0.199 | 3.433 | 17.23× |
| 1,000 | 10 | 0.440 | 6.944 | 15.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
