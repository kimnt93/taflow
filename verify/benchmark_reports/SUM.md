# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.43M | 0.006 | 178.91M | 0.033 | 4.84× | 5.87× |
| 10,000 | 0.038 | 262.71M | 0.036 | 278.68M | 0.051 | 1.33× | 1.41× |
| 100,000 | 0.328 | 304.94M | 0.306 | 326.40M | 0.235 | 0.72× | 0.77× |
| 1,000,000 | 3.784 | 264.24M | 3.271 | 305.68M | 2.046 | 0.54× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.109 | 1.69× |
| 1 | 5 | 0.283 | 0.547 | 1.93× |
| 1 | 10 | 0.549 | 1.056 | 1.92× |
| 10 | 1 | 0.050 | 0.093 | 1.86× |
| 10 | 5 | 0.263 | 0.529 | 2.01× |
| 10 | 10 | 0.576 | 1.127 | 1.96× |
| 100 | 1 | 0.052 | 0.100 | 1.92× |
| 100 | 5 | 0.237 | 0.485 | 2.04× |
| 100 | 10 | 0.590 | 1.053 | 1.79× |
| 1,000 | 1 | 0.059 | 0.096 | 1.64× |
| 1,000 | 5 | 0.246 | 0.483 | 1.96× |
| 1,000 | 10 | 0.547 | 1.056 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
