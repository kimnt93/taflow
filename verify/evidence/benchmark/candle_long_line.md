# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.99M | 0.009 | 112.56M | 0.038 | 2.99× | 4.26× |
| 10,000 | 0.147 | 67.80M | 0.139 | 71.81M | 0.175 | 1.19× | 1.26× |
| 100,000 | 1.494 | 66.92M | 1.480 | 67.58M | 1.635 | 1.09× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.123 | 1.13× |
| 1 | 5 | 0.256 | 0.454 | 1.77× |
| 1 | 10 | 0.400 | 0.999 | 2.50× |
| 10 | 1 | 0.048 | 0.094 | 1.96× |
| 10 | 5 | 0.181 | 0.440 | 2.43× |
| 10 | 10 | 0.388 | 0.912 | 2.35× |
| 100 | 1 | 0.042 | 0.095 | 2.30× |
| 100 | 5 | 0.226 | 0.490 | 2.16× |
| 100 | 10 | 0.384 | 0.930 | 2.42× |
| 1,000 | 1 | 0.055 | 0.106 | 1.92× |
| 1,000 | 5 | 0.193 | 0.507 | 2.62× |
| 1,000 | 10 | 0.438 | 1.110 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
