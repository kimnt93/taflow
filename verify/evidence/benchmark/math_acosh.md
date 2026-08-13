# MathAcosh benchmark (`numpy.arccosh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.71M | 0.028 | 35.12M | 0.023 | 0.68× | 0.80× |
| 10,000 | 0.250 | 40.01M | 0.239 | 41.92M | 0.116 | 0.46× | 0.48× |
| 100,000 | 2.255 | 44.35M | 2.191 | 45.64M | 1.060 | 0.47× | 0.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.156 | 0.091 | 0.58× |
| 1 | 5 | 0.345 | 0.276 | 0.80× |
| 1 | 10 | 0.563 | 0.548 | 0.97× |
| 10 | 1 | 0.061 | 0.055 | 0.89× |
| 10 | 5 | 0.276 | 0.278 | 1.01× |
| 10 | 10 | 0.563 | 0.566 | 1.00× |
| 100 | 1 | 0.062 | 0.056 | 0.90× |
| 100 | 5 | 0.272 | 0.280 | 1.03× |
| 100 | 10 | 0.605 | 0.614 | 1.01× |
| 1,000 | 1 | 0.095 | 0.073 | 0.76× |
| 1,000 | 5 | 0.280 | 0.339 | 1.21× |
| 1,000 | 10 | 0.593 | 0.723 | 1.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
