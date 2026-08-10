# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.27M | 0.017 | 57.62M | 0.040 | 1.88× | 2.29× |
| 10,000 | 0.150 | 66.86M | 0.158 | 63.22M | 0.164 | 1.10× | 1.04× |
| 100,000 | 1.567 | 63.83M | 1.490 | 67.11M | 1.143 | 0.73× | 0.77× |
| 1,000,000 | 14.948 | 66.90M | 15.169 | 65.92M | 11.573 | 0.77× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.132 | 1.15× |
| 1 | 5 | 0.336 | 0.734 | 2.18× |
| 1 | 10 | 0.753 | 1.135 | 1.51× |
| 10 | 1 | 0.059 | 0.092 | 1.56× |
| 10 | 5 | 0.281 | 0.576 | 2.05× |
| 10 | 10 | 0.602 | 1.125 | 1.87× |
| 100 | 1 | 0.077 | 0.126 | 1.63× |
| 100 | 5 | 0.326 | 0.576 | 1.77× |
| 100 | 10 | 0.713 | 1.208 | 1.69× |
| 1,000 | 1 | 0.075 | 0.108 | 1.44× |
| 1,000 | 5 | 0.346 | 0.626 | 1.81× |
| 1,000 | 10 | 0.728 | 1.246 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
