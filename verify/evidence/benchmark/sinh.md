# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.68M | 0.007 | 148.09M | 0.037 | 4.70× | 5.45× |
| 10,000 | 0.065 | 153.69M | 0.063 | 157.86M | 0.090 | 1.39× | 1.42× |
| 100,000 | 0.641 | 156.01M | 0.590 | 169.59M | 0.650 | 1.01× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.117 | 1.39× |
| 1 | 5 | 0.356 | 0.491 | 1.38× |
| 1 | 10 | 0.379 | 0.941 | 2.48× |
| 10 | 1 | 0.051 | 0.098 | 1.93× |
| 10 | 5 | 0.200 | 0.453 | 2.26× |
| 10 | 10 | 0.396 | 0.929 | 2.35× |
| 100 | 1 | 0.056 | 0.107 | 1.91× |
| 100 | 5 | 0.212 | 0.463 | 2.19× |
| 100 | 10 | 0.395 | 0.884 | 2.24× |
| 1,000 | 1 | 0.056 | 0.107 | 1.92× |
| 1,000 | 5 | 0.191 | 0.461 | 2.41× |
| 1,000 | 10 | 0.478 | 0.934 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
