# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.96M | 0.013 | 78.14M | 0.048 | 3.54× | 3.74× |
| 10,000 | 0.100 | 100.21M | 0.088 | 113.22M | 0.116 | 1.16× | 1.31× |
| 100,000 | 0.917 | 109.07M | 0.903 | 110.75M | 0.790 | 0.86× | 0.87× |
| 1,000,000 | 19.180 | 52.14M | 16.995 | 58.84M | 13.443 | 0.70× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.148 | 1.53× |
| 1 | 5 | 0.277 | 0.579 | 2.09× |
| 1 | 10 | 0.513 | 1.117 | 2.18× |
| 10 | 1 | 0.052 | 0.108 | 2.09× |
| 10 | 5 | 0.226 | 0.511 | 2.26× |
| 10 | 10 | 0.481 | 1.107 | 2.30× |
| 100 | 1 | 0.057 | 0.106 | 1.87× |
| 100 | 5 | 0.241 | 0.530 | 2.20× |
| 100 | 10 | 0.502 | 1.093 | 2.18× |
| 1,000 | 1 | 0.072 | 0.131 | 1.81× |
| 1,000 | 5 | 0.284 | 0.581 | 2.04× |
| 1,000 | 10 | 0.558 | 1.218 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
