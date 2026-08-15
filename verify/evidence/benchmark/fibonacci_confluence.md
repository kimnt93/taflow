# FibonacciConfluence benchmark (`FibConfluence` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.858 | 1.17M | 0.857 | 1.17M | 1.883 | 2.20× | 2.20× |
| 10,000 | 8.777 | 1.14M | 8.630 | 1.16M | 17.732 | 2.02× | 2.05× |
| 100,000 | 87.894 | 1.14M | 95.016 | 1.05M | 181.895 | 2.07× | 1.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.230 | 4.14× |
| 1 | 5 | 0.232 | 0.844 | 3.64× |
| 1 | 10 | 0.382 | 1.940 | 5.08× |
| 10 | 1 | 0.047 | 0.173 | 3.64× |
| 10 | 5 | 0.180 | 0.854 | 4.76× |
| 10 | 10 | 0.386 | 1.963 | 5.08× |
| 100 | 1 | 0.080 | 0.287 | 3.61× |
| 100 | 5 | 0.205 | 1.394 | 6.81× |
| 100 | 10 | 0.459 | 3.083 | 6.72× |
| 1,000 | 1 | 0.933 | 2.234 | 2.39× |
| 1,000 | 5 | 1.065 | 10.463 | 9.83× |
| 1,000 | 10 | 1.649 | 21.873 | 13.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
