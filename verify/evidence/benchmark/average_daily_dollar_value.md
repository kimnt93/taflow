# AverageDailyDollarValue benchmark (`rolling average dollar volume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.83M | 0.009 | 115.86M | 0.082 | 8.62× | 9.53× |
| 10,000 | 0.054 | 184.02M | 0.052 | 194.08M | 0.284 | 5.23× | 5.51× |
| 100,000 | 0.490 | 204.01M | 0.446 | 224.28M | 2.110 | 4.30× | 4.73× |
| 1,000,000 | 5.130 | 194.92M | 4.731 | 211.35M | 21.879 | 4.26× | 4.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.124 | 1.17× |
| 1 | 5 | 0.484 | 0.552 | 1.14× |
| 1 | 10 | 0.467 | 1.074 | 2.30× |
| 10 | 1 | 0.048 | 0.099 | 2.06× |
| 10 | 5 | 0.217 | 0.501 | 2.31× |
| 10 | 10 | 0.486 | 1.056 | 2.17× |
| 100 | 1 | 0.053 | 0.146 | 2.77× |
| 100 | 5 | 0.224 | 0.683 | 3.05× |
| 100 | 10 | 0.483 | 1.424 | 2.95× |
| 1,000 | 1 | 0.063 | 0.155 | 2.47× |
| 1,000 | 5 | 0.227 | 0.784 | 3.46× |
| 1,000 | 10 | 0.474 | 1.586 | 3.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
