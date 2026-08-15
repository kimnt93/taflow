# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.38M | 0.009 | 113.79M | 0.367 | 33.17× | 41.76× |
| 10,000 | 0.066 | 150.39M | 0.069 | 145.09M | 2.335 | 35.12× | 33.88× |
| 100,000 | 0.615 | 162.63M | 0.745 | 134.21M | 24.764 | 40.27× | 33.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.276 | 2.01× |
| 1 | 5 | 0.245 | 1.153 | 4.70× |
| 1 | 10 | 0.420 | 2.579 | 6.13× |
| 10 | 1 | 0.048 | 0.235 | 4.90× |
| 10 | 5 | 0.197 | 1.113 | 5.64× |
| 10 | 10 | 0.413 | 2.421 | 5.87× |
| 100 | 1 | 0.047 | 0.246 | 5.25× |
| 100 | 5 | 0.206 | 1.387 | 6.75× |
| 100 | 10 | 0.422 | 2.647 | 6.27× |
| 1,000 | 1 | 0.051 | 0.457 | 8.89× |
| 1,000 | 5 | 0.202 | 2.488 | 12.33× |
| 1,000 | 10 | 0.461 | 4.829 | 10.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
