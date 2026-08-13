# Ichimoku benchmark (`causal ichimoku components` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.325 | 3.08M | 0.308 | 3.25M | 0.426 | 1.31× | 1.38× |
| 10,000 | 3.075 | 3.25M | 3.300 | 3.03M | 2.746 | 0.89× | 0.83× |
| 100,000 | 31.149 | 3.21M | 30.832 | 3.24M | 23.117 | 0.74× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.159 | 1.37× |
| 1 | 5 | 0.387 | 0.650 | 1.68× |
| 1 | 10 | 0.658 | 1.217 | 1.85× |
| 10 | 1 | 0.076 | 0.210 | 2.77× |
| 10 | 5 | 0.365 | 0.970 | 2.66× |
| 10 | 10 | 0.694 | 1.974 | 2.85× |
| 100 | 1 | 0.105 | 0.321 | 3.05× |
| 100 | 5 | 0.332 | 1.756 | 5.28× |
| 100 | 10 | 0.719 | 3.667 | 5.10× |
| 1,000 | 1 | 0.394 | 0.556 | 1.41× |
| 1,000 | 5 | 0.609 | 2.008 | 3.30× |
| 1,000 | 10 | 1.002 | 4.354 | 4.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
