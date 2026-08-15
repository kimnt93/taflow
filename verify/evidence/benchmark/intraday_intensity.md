# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.07M | 0.004 | 239.50M | 0.225 | 31.72× | 53.86× |
| 10,000 | 0.034 | 293.37M | 0.029 | 339.66M | 1.319 | 38.68× | 44.79× |
| 100,000 | 0.290 | 345.04M | 0.273 | 366.76M | 12.181 | 42.03× | 44.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.217 | 3.03× |
| 1 | 5 | 0.311 | 0.846 | 2.72× |
| 1 | 10 | 0.407 | 1.637 | 4.02× |
| 10 | 1 | 0.049 | 0.172 | 3.53× |
| 10 | 5 | 0.210 | 1.082 | 5.14× |
| 10 | 10 | 0.403 | 1.635 | 4.05× |
| 100 | 1 | 0.052 | 0.186 | 3.59× |
| 100 | 5 | 0.235 | 1.167 | 4.97× |
| 100 | 10 | 0.429 | 1.796 | 4.18× |
| 1,000 | 1 | 0.051 | 0.293 | 5.79× |
| 1,000 | 5 | 0.227 | 1.718 | 7.57× |
| 1,000 | 10 | 0.428 | 2.935 | 6.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
