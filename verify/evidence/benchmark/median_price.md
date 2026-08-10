# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.75M | 0.005 | 185.01M | 0.030 | 4.22× | 5.62× |
| 10,000 | 0.021 | 472.32M | 0.019 | 521.73M | 0.037 | 1.74× | 1.92× |
| 100,000 | 0.167 | 599.53M | 0.147 | 682.25M | 0.075 | 0.45× | 0.51× |
| 1,000,000 | 2.573 | 388.64M | 1.882 | 531.43M | 1.459 | 0.57× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.105 | 0.75× |
| 1 | 5 | 0.283 | 0.470 | 1.66× |
| 1 | 10 | 0.504 | 1.075 | 2.13× |
| 10 | 1 | 0.059 | 0.095 | 1.61× |
| 10 | 5 | 0.262 | 0.475 | 1.81× |
| 10 | 10 | 0.516 | 1.003 | 1.95× |
| 100 | 1 | 0.062 | 0.095 | 1.53× |
| 100 | 5 | 0.331 | 0.495 | 1.50× |
| 100 | 10 | 0.544 | 0.929 | 1.71× |
| 1,000 | 1 | 0.057 | 0.087 | 1.54× |
| 1,000 | 5 | 0.262 | 0.574 | 2.19× |
| 1,000 | 10 | 0.556 | 0.996 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
