# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.119 | 8.43M | 0.108 | 9.22M | 0.043 | 0.36× | 0.40× |
| 10,000 | 0.968 | 10.33M | 1.003 | 9.97M | 0.178 | 0.18× | 0.18× |
| 100,000 | 9.696 | 10.31M | 9.385 | 10.65M | 1.460 | 0.15× | 0.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.116 | 0.91× |
| 1 | 5 | 0.362 | 0.457 | 1.26× |
| 1 | 10 | 0.627 | 0.922 | 1.47× |
| 10 | 1 | 0.068 | 0.086 | 1.27× |
| 10 | 5 | 0.318 | 0.431 | 1.36× |
| 10 | 10 | 0.653 | 0.918 | 1.41× |
| 100 | 1 | 0.078 | 0.096 | 1.23× |
| 100 | 5 | 0.312 | 0.435 | 1.40× |
| 100 | 10 | 0.713 | 0.980 | 1.37× |
| 1,000 | 1 | 0.177 | 0.122 | 0.69× |
| 1,000 | 5 | 0.351 | 0.522 | 1.49× |
| 1,000 | 10 | 0.725 | 1.086 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
