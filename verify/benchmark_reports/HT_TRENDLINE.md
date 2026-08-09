# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.57M | 0.072 | 13.82M | 0.082 | 1.20× | 1.14× |
| 10,000 | 0.704 | 14.21M | 0.672 | 14.89M | 0.628 | 0.89× | 0.93× |
| 100,000 | 6.989 | 14.31M | 7.064 | 14.16M | 5.564 | 0.80× | 0.79× |
| 1,000,000 | 70.627 | 14.16M | 71.797 | 13.93M | 57.093 | 0.81× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.166 | 1.13× |
| 1 | 5 | 0.272 | 0.439 | 1.62× |
| 1 | 10 | 0.450 | 0.912 | 2.03× |
| 10 | 1 | 0.049 | 0.090 | 1.83× |
| 10 | 5 | 0.205 | 0.417 | 2.03× |
| 10 | 10 | 0.447 | 0.877 | 1.96× |
| 100 | 1 | 0.055 | 0.095 | 1.72× |
| 100 | 5 | 0.266 | 0.499 | 1.87× |
| 100 | 10 | 0.463 | 0.929 | 2.00× |
| 1,000 | 1 | 0.117 | 0.147 | 1.25× |
| 1,000 | 5 | 0.251 | 0.719 | 2.87× |
| 1,000 | 10 | 0.503 | 1.543 | 3.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
