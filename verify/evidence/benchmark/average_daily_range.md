# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.80M | 0.007 | 146.24M | 0.409 | 40.04× | 59.87× |
| 10,000 | 0.058 | 172.46M | 0.053 | 190.31M | 2.477 | 42.71× | 47.13× |
| 100,000 | 0.527 | 189.65M | 0.507 | 197.29M | 22.892 | 43.41× | 45.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.356 | 3.39× |
| 1 | 5 | 0.316 | 1.293 | 4.09× |
| 1 | 10 | 0.403 | 2.716 | 6.75× |
| 10 | 1 | 0.049 | 0.258 | 5.23× |
| 10 | 5 | 0.200 | 1.436 | 7.17× |
| 10 | 10 | 0.427 | 2.839 | 6.65× |
| 100 | 1 | 0.044 | 0.274 | 6.27× |
| 100 | 5 | 0.196 | 1.575 | 8.05× |
| 100 | 10 | 0.481 | 2.837 | 5.90× |
| 1,000 | 1 | 0.060 | 0.484 | 8.05× |
| 1,000 | 5 | 0.205 | 2.614 | 12.73× |
| 1,000 | 10 | 0.457 | 5.459 | 11.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
