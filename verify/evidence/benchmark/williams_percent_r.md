# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.93M | 0.006 | 161.63M | 0.034 | 4.43× | 5.52× |
| 10,000 | 0.052 | 191.50M | 0.056 | 177.02M | 0.108 | 2.07× | 1.92× |
| 100,000 | 0.489 | 204.54M | 0.466 | 214.71M | 0.790 | 1.62× | 1.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.135 | 1.59× |
| 1 | 5 | 0.322 | 0.595 | 1.85× |
| 1 | 10 | 0.395 | 0.924 | 2.34× |
| 10 | 1 | 0.044 | 0.085 | 1.94× |
| 10 | 5 | 0.182 | 0.431 | 2.37× |
| 10 | 10 | 0.372 | 0.883 | 2.37× |
| 100 | 1 | 0.039 | 0.098 | 2.49× |
| 100 | 5 | 0.184 | 0.409 | 2.23× |
| 100 | 10 | 0.391 | 0.936 | 2.40× |
| 1,000 | 1 | 0.048 | 0.099 | 2.06× |
| 1,000 | 5 | 0.191 | 0.490 | 2.56× |
| 1,000 | 10 | 0.491 | 0.997 | 2.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
