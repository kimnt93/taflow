# HighLowIndex benchmark (`HighLowIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.51M | 0.009 | 112.64M | 8.728 | 833.59× | 983.15× |
| 10,000 | 0.062 | 162.50M | 0.057 | 176.82M | 89.095 | 1447.82× | 1575.35× |
| 100,000 | 0.550 | 181.73M | 0.528 | 189.35M | 872.785 | 1586.15× | 1652.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.176 | 0.279 | 1.59× |
| 1 | 5 | 0.408 | 1.564 | 3.83× |
| 1 | 10 | 0.489 | 2.657 | 5.44× |
| 10 | 1 | 0.054 | 0.327 | 6.09× |
| 10 | 5 | 0.231 | 1.602 | 6.93× |
| 10 | 10 | 0.495 | 3.557 | 7.18× |
| 100 | 1 | 0.052 | 1.132 | 21.56× |
| 100 | 5 | 0.264 | 5.727 | 21.67× |
| 100 | 10 | 0.514 | 11.808 | 22.98× |
| 1,000 | 1 | 0.064 | 9.289 | 144.07× |
| 1,000 | 5 | 0.402 | 47.670 | 118.57× |
| 1,000 | 10 | 0.585 | 94.171 | 160.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
