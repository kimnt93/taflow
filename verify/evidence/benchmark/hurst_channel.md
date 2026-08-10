# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.71M | 0.043 | 23.19M | 0.672 | 14.58× | 15.58× |
| 10,000 | 0.436 | 22.92M | 0.402 | 24.90M | 4.676 | 10.72× | 11.65× |
| 100,000 | 4.216 | 23.72M | 4.040 | 24.75M | 48.324 | 11.46× | 11.96× |
| 1,000,000 | 41.938 | 23.84M | 51.506 | 19.42M | 555.038 | 13.23× | 10.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.308 | 4.42× |
| 1 | 5 | 0.292 | 1.405 | 4.80× |
| 1 | 10 | 0.507 | 2.712 | 5.35× |
| 10 | 1 | 0.063 | 0.267 | 4.22× |
| 10 | 5 | 0.265 | 1.510 | 5.70× |
| 10 | 10 | 0.524 | 2.862 | 5.46× |
| 100 | 1 | 0.057 | 0.328 | 5.73× |
| 100 | 5 | 0.265 | 1.764 | 6.66× |
| 100 | 10 | 0.564 | 3.399 | 6.03× |
| 1,000 | 1 | 0.112 | 0.955 | 8.51× |
| 1,000 | 5 | 0.294 | 3.996 | 13.59× |
| 1,000 | 10 | 0.658 | 7.851 | 11.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
