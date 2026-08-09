# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.40M | 0.016 | 63.73M | 0.047 | 2.83× | 2.99× |
| 10,000 | 0.125 | 80.09M | 0.114 | 87.77M | 0.133 | 1.07× | 1.17× |
| 100,000 | 1.177 | 84.97M | 1.140 | 87.74M | 1.073 | 0.91× | 0.94× |
| 1,000,000 | 11.186 | 89.40M | 11.120 | 89.93M | 10.261 | 0.92× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.129 | 1.42× |
| 1 | 5 | 0.339 | 0.562 | 1.66× |
| 1 | 10 | 0.563 | 1.015 | 1.80× |
| 10 | 1 | 0.049 | 0.094 | 1.91× |
| 10 | 5 | 0.233 | 0.488 | 2.09× |
| 10 | 10 | 0.514 | 1.018 | 1.98× |
| 100 | 1 | 0.055 | 0.101 | 1.83× |
| 100 | 5 | 0.256 | 0.487 | 1.90× |
| 100 | 10 | 0.519 | 1.020 | 1.97× |
| 1,000 | 1 | 0.072 | 0.122 | 1.69× |
| 1,000 | 5 | 0.307 | 0.587 | 1.91× |
| 1,000 | 10 | 0.531 | 1.097 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
