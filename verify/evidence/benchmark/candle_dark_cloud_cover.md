# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.11M | 0.002 | 436.50M | 0.034 | 5.77× | 14.81× |
| 10,000 | 0.064 | 155.55M | 0.061 | 163.84M | 0.114 | 1.77× | 1.87× |
| 100,000 | 0.736 | 135.84M | 0.710 | 140.94M | 0.832 | 1.13× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.109 | 0.71× |
| 1 | 5 | 0.273 | 0.517 | 1.89× |
| 1 | 10 | 0.388 | 0.919 | 2.37× |
| 10 | 1 | 0.043 | 0.093 | 2.14× |
| 10 | 5 | 0.186 | 0.446 | 2.41× |
| 10 | 10 | 0.391 | 0.989 | 2.53× |
| 100 | 1 | 0.044 | 0.088 | 1.99× |
| 100 | 5 | 0.172 | 0.440 | 2.56× |
| 100 | 10 | 0.386 | 0.935 | 2.43× |
| 1,000 | 1 | 0.061 | 0.116 | 1.90× |
| 1,000 | 5 | 0.199 | 0.485 | 2.44× |
| 1,000 | 10 | 0.402 | 1.013 | 2.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
