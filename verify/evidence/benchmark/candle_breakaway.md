# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.25M | 0.004 | 257.89M | 0.032 | 4.43× | 8.15× |
| 10,000 | 0.081 | 123.68M | 0.078 | 128.56M | 0.089 | 1.10× | 1.14× |
| 100,000 | 0.884 | 113.08M | 0.881 | 113.47M | 0.650 | 0.73× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.106 | 1.08× |
| 1 | 5 | 0.245 | 0.436 | 1.78× |
| 1 | 10 | 0.418 | 0.904 | 2.16× |
| 10 | 1 | 0.042 | 0.086 | 2.06× |
| 10 | 5 | 0.180 | 0.433 | 2.40× |
| 10 | 10 | 0.400 | 0.935 | 2.34× |
| 100 | 1 | 0.042 | 0.089 | 2.14× |
| 100 | 5 | 0.213 | 0.433 | 2.03× |
| 100 | 10 | 0.391 | 0.910 | 2.33× |
| 1,000 | 1 | 0.049 | 0.093 | 1.88× |
| 1,000 | 5 | 0.205 | 0.498 | 2.43× |
| 1,000 | 10 | 0.417 | 0.979 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
