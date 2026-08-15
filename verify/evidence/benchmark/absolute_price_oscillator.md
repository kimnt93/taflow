# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.08M | 0.005 | 203.08M | 0.042 | 7.34× | 8.61× |
| 10,000 | 0.046 | 218.32M | 0.038 | 264.71M | 0.078 | 1.70× | 2.06× |
| 100,000 | 0.347 | 288.22M | 0.311 | 321.90M | 0.475 | 1.37× | 1.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.112 | 1.42× |
| 1 | 5 | 0.267 | 0.567 | 2.13× |
| 1 | 10 | 0.410 | 1.031 | 2.51× |
| 10 | 1 | 0.048 | 0.097 | 2.00× |
| 10 | 5 | 0.201 | 0.560 | 2.78× |
| 10 | 10 | 0.454 | 1.154 | 2.54× |
| 100 | 1 | 0.047 | 0.094 | 2.01× |
| 100 | 5 | 0.196 | 0.531 | 2.71× |
| 100 | 10 | 0.432 | 1.114 | 2.58× |
| 1,000 | 1 | 0.057 | 0.117 | 2.07× |
| 1,000 | 5 | 0.228 | 0.617 | 2.70× |
| 1,000 | 10 | 0.415 | 1.155 | 2.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
