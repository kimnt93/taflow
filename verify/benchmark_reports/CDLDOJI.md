# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.30M | 0.008 | 129.96M | 0.030 | 3.36× | 3.92× |
| 10,000 | 0.032 | 311.89M | 0.029 | 344.09M | 0.051 | 1.58× | 1.74× |
| 100,000 | 0.260 | 384.10M | 0.285 | 351.13M | 0.234 | 0.90× | 0.82× |
| 1,000,000 | 3.239 | 308.71M | 3.167 | 315.78M | 2.624 | 0.81× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.132 | 1.47× |
| 1 | 5 | 0.292 | 0.517 | 1.77× |
| 1 | 10 | 0.556 | 0.994 | 1.79× |
| 10 | 1 | 0.053 | 0.096 | 1.81× |
| 10 | 5 | 0.273 | 0.528 | 1.93× |
| 10 | 10 | 0.593 | 1.041 | 1.76× |
| 100 | 1 | 0.059 | 0.104 | 1.75× |
| 100 | 5 | 0.286 | 0.495 | 1.73× |
| 100 | 10 | 0.586 | 0.980 | 1.67× |
| 1,000 | 1 | 0.080 | 0.134 | 1.68× |
| 1,000 | 5 | 0.324 | 0.520 | 1.60× |
| 1,000 | 10 | 0.593 | 1.072 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
