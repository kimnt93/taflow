# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.73M | 0.010 | 100.66M | 0.037 | 3.55× | 3.77× |
| 10,000 | 0.082 | 122.59M | 0.077 | 129.39M | 0.088 | 1.08× | 1.14× |
| 100,000 | 0.787 | 127.13M | 0.753 | 132.73M | 0.586 | 0.74× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.136 | 1.35× |
| 1 | 5 | 0.299 | 0.477 | 1.60× |
| 1 | 10 | 0.474 | 0.983 | 2.07× |
| 10 | 1 | 0.053 | 0.092 | 1.74× |
| 10 | 5 | 0.215 | 0.442 | 2.05× |
| 10 | 10 | 0.463 | 0.947 | 2.04× |
| 100 | 1 | 0.048 | 0.098 | 2.05× |
| 100 | 5 | 0.234 | 0.463 | 1.98× |
| 100 | 10 | 0.470 | 0.914 | 1.94× |
| 1,000 | 1 | 0.056 | 0.094 | 1.67× |
| 1,000 | 5 | 0.228 | 0.470 | 2.06× |
| 1,000 | 10 | 0.518 | 1.064 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
