# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.00M | 0.005 | 196.09M | 0.041 | 5.06× | 7.94× |
| 10,000 | 0.056 | 178.69M | 0.050 | 198.65M | 0.124 | 2.22× | 2.46× |
| 100,000 | 0.623 | 160.64M | 0.633 | 158.05M | 0.912 | 1.46× | 1.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.144 | 2.18× |
| 1 | 5 | 0.282 | 0.463 | 1.64× |
| 1 | 10 | 0.400 | 1.013 | 2.54× |
| 10 | 1 | 0.050 | 0.107 | 2.15× |
| 10 | 5 | 0.215 | 0.470 | 2.18× |
| 10 | 10 | 0.403 | 1.013 | 2.51× |
| 100 | 1 | 0.040 | 0.091 | 2.29× |
| 100 | 5 | 0.191 | 0.543 | 2.84× |
| 100 | 10 | 0.410 | 0.963 | 2.35× |
| 1,000 | 1 | 0.050 | 0.101 | 2.00× |
| 1,000 | 5 | 0.188 | 0.504 | 2.68× |
| 1,000 | 10 | 0.469 | 1.082 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
