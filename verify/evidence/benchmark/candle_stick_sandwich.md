# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.69M | 0.008 | 124.16M | 0.031 | 2.90× | 3.84× |
| 10,000 | 0.048 | 208.21M | 0.052 | 193.89M | 0.089 | 1.84× | 1.72× |
| 100,000 | 0.684 | 146.13M | 0.547 | 182.94M | 0.607 | 0.89× | 1.11× |
| 1,000,000 | 6.127 | 163.21M | 6.193 | 161.48M | 6.681 | 1.09× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.107 | 0.99× |
| 1 | 5 | 0.376 | 0.451 | 1.20× |
| 1 | 10 | 0.535 | 0.919 | 1.72× |
| 10 | 1 | 0.052 | 0.088 | 1.68× |
| 10 | 5 | 0.257 | 0.427 | 1.66× |
| 10 | 10 | 0.531 | 0.909 | 1.71× |
| 100 | 1 | 0.056 | 0.087 | 1.54× |
| 100 | 5 | 0.255 | 0.444 | 1.74× |
| 100 | 10 | 0.528 | 0.860 | 1.63× |
| 1,000 | 1 | 0.063 | 0.098 | 1.55× |
| 1,000 | 5 | 0.285 | 0.454 | 1.59× |
| 1,000 | 10 | 0.567 | 0.971 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
