# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.02M | 0.003 | 344.93M | 0.031 | 5.08× | 10.62× |
| 10,000 | 0.075 | 133.14M | 0.070 | 142.75M | 0.097 | 1.29× | 1.38× |
| 100,000 | 0.787 | 127.01M | 0.818 | 122.29M | 0.563 | 0.71× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.116 | 1.59× |
| 1 | 5 | 0.258 | 0.460 | 1.78× |
| 1 | 10 | 0.382 | 0.907 | 2.38× |
| 10 | 1 | 0.042 | 0.087 | 2.06× |
| 10 | 5 | 0.175 | 0.414 | 2.37× |
| 10 | 10 | 0.384 | 0.879 | 2.29× |
| 100 | 1 | 0.040 | 0.085 | 2.14× |
| 100 | 5 | 0.180 | 0.409 | 2.28× |
| 100 | 10 | 0.389 | 0.875 | 2.25× |
| 1,000 | 1 | 0.049 | 0.089 | 1.82× |
| 1,000 | 5 | 0.188 | 0.447 | 2.38× |
| 1,000 | 10 | 0.423 | 0.910 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
