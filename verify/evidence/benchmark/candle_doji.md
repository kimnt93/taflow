# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.54M | 0.010 | 103.85M | 0.033 | 2.59× | 3.47× |
| 10,000 | 0.043 | 231.60M | 0.041 | 243.86M | 0.056 | 1.30× | 1.37× |
| 100,000 | 0.378 | 264.69M | 0.354 | 282.21M | 0.258 | 0.68× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.126 | 1.03× |
| 1 | 5 | 0.321 | 0.443 | 1.38× |
| 1 | 10 | 0.523 | 0.929 | 1.78× |
| 10 | 1 | 0.063 | 0.092 | 1.45× |
| 10 | 5 | 0.310 | 0.468 | 1.51× |
| 10 | 10 | 0.543 | 0.857 | 1.58× |
| 100 | 1 | 0.057 | 0.092 | 1.62× |
| 100 | 5 | 0.280 | 0.462 | 1.65× |
| 100 | 10 | 0.552 | 0.901 | 1.63× |
| 1,000 | 1 | 0.059 | 0.095 | 1.60× |
| 1,000 | 5 | 0.263 | 0.447 | 1.70× |
| 1,000 | 10 | 0.619 | 1.016 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
