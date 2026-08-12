# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.17M | 0.009 | 108.46M | 0.031 | 2.55× | 3.40× |
| 10,000 | 0.088 | 113.94M | 0.084 | 118.76M | 0.096 | 1.09× | 1.14× |
| 100,000 | 0.956 | 104.58M | 0.924 | 108.17M | 0.666 | 0.70× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.104 | 1.11× |
| 1 | 5 | 0.355 | 0.483 | 1.36× |
| 1 | 10 | 0.537 | 0.964 | 1.80× |
| 10 | 1 | 0.059 | 0.088 | 1.50× |
| 10 | 5 | 0.252 | 0.425 | 1.69× |
| 10 | 10 | 0.524 | 0.909 | 1.73× |
| 100 | 1 | 0.062 | 0.097 | 1.57× |
| 100 | 5 | 0.275 | 0.426 | 1.55× |
| 100 | 10 | 0.549 | 0.922 | 1.68× |
| 1,000 | 1 | 0.076 | 0.100 | 1.32× |
| 1,000 | 5 | 0.301 | 0.533 | 1.77× |
| 1,000 | 10 | 0.585 | 0.963 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
