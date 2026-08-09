# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.37M | 0.005 | 199.47M | 0.037 | 6.48× | 7.46× |
| 10,000 | 0.028 | 357.92M | 0.025 | 406.28M | 0.045 | 1.62× | 1.83× |
| 100,000 | 0.244 | 409.54M | 0.215 | 464.65M | 0.144 | 0.59× | 0.67× |
| 1,000,000 | 2.633 | 379.83M | 2.181 | 458.47M | 1.263 | 0.48× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.171 | 1.25× |
| 1 | 5 | 0.293 | 0.528 | 1.80× |
| 1 | 10 | 0.486 | 0.979 | 2.01× |
| 10 | 1 | 0.056 | 0.095 | 1.71× |
| 10 | 5 | 0.232 | 0.471 | 2.03× |
| 10 | 10 | 0.516 | 0.976 | 1.89× |
| 100 | 1 | 0.050 | 0.093 | 1.85× |
| 100 | 5 | 0.251 | 0.456 | 1.82× |
| 100 | 10 | 0.533 | 1.018 | 1.91× |
| 1,000 | 1 | 0.052 | 0.094 | 1.81× |
| 1,000 | 5 | 0.241 | 0.456 | 1.89× |
| 1,000 | 10 | 0.526 | 0.982 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
