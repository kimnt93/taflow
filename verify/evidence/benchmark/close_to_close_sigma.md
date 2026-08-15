# CloseToCloseSigma benchmark (`annualized close-to-close volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.51M | 0.022 | 44.95M | 0.147 | 6.69× | 6.61× |
| 10,000 | 0.202 | 49.46M | 0.191 | 52.45M | 0.685 | 3.39× | 3.59× |
| 100,000 | 1.860 | 53.77M | 1.873 | 53.40M | 7.549 | 4.06× | 4.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.166 | 2.96× |
| 1 | 5 | 0.242 | 0.722 | 2.99× |
| 1 | 10 | 0.379 | 1.268 | 3.35× |
| 10 | 1 | 0.050 | 0.120 | 2.38× |
| 10 | 5 | 0.211 | 0.583 | 2.76× |
| 10 | 10 | 0.387 | 1.182 | 3.05× |
| 100 | 1 | 0.044 | 0.180 | 4.08× |
| 100 | 5 | 0.234 | 0.878 | 3.76× |
| 100 | 10 | 0.423 | 1.745 | 4.13× |
| 1,000 | 1 | 0.069 | 0.243 | 3.52× |
| 1,000 | 5 | 0.251 | 1.196 | 4.76× |
| 1,000 | 10 | 0.450 | 2.310 | 5.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
