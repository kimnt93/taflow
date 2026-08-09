# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.70M | 0.007 | 134.94M | 0.029 | 3.15× | 3.98× |
| 10,000 | 0.075 | 132.85M | 0.073 | 137.70M | 0.088 | 1.16× | 1.21× |
| 100,000 | 0.834 | 119.87M | 0.852 | 117.41M | 0.634 | 0.76× | 0.74× |
| 1,000,000 | 8.715 | 114.75M | 9.142 | 109.39M | 7.221 | 0.83× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.108 | 0.72× |
| 1 | 5 | 0.359 | 0.528 | 1.47× |
| 1 | 10 | 0.517 | 0.953 | 1.84× |
| 10 | 1 | 0.054 | 0.094 | 1.73× |
| 10 | 5 | 0.232 | 0.436 | 1.88× |
| 10 | 10 | 0.530 | 0.929 | 1.75× |
| 100 | 1 | 0.054 | 0.089 | 1.65× |
| 100 | 5 | 0.249 | 0.468 | 1.88× |
| 100 | 10 | 0.519 | 0.956 | 1.84× |
| 1,000 | 1 | 0.064 | 0.100 | 1.56× |
| 1,000 | 5 | 0.261 | 0.490 | 1.88× |
| 1,000 | 10 | 0.564 | 1.067 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
