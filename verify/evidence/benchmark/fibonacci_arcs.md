# FibonacciArcs benchmark (`FibArcs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.58M | 0.018 | 56.34M | 0.474 | 15.93× | 26.73× |
| 10,000 | 0.156 | 64.26M | 0.145 | 69.10M | 3.620 | 23.26× | 25.02× |
| 100,000 | 1.498 | 66.78M | 1.379 | 72.53M | 38.873 | 25.96× | 28.20× |
| 1,000,000 | 15.405 | 64.91M | 13.775 | 72.60M | 441.309 | 28.65× | 32.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.203 | 2.74× |
| 1 | 5 | 0.275 | 0.843 | 3.06× |
| 1 | 10 | 0.474 | 1.810 | 3.82× |
| 10 | 1 | 0.052 | 0.170 | 3.30× |
| 10 | 5 | 0.229 | 1.074 | 4.70× |
| 10 | 10 | 0.476 | 1.946 | 4.08× |
| 100 | 1 | 0.059 | 0.219 | 3.71× |
| 100 | 5 | 0.266 | 1.054 | 3.97× |
| 100 | 10 | 0.559 | 2.328 | 4.17× |
| 1,000 | 1 | 0.074 | 0.739 | 10.03× |
| 1,000 | 5 | 0.262 | 3.105 | 11.83× |
| 1,000 | 10 | 0.527 | 7.251 | 13.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
