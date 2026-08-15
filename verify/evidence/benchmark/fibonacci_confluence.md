# FibonacciConfluence benchmark (`FibConfluence` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.906 | 1.10M | 0.873 | 1.15M | 1.919 | 2.12× | 2.20× |
| 10,000 | 8.871 | 1.13M | 8.962 | 1.12M | 19.651 | 2.22× | 2.19× |
| 100,000 | 89.326 | 1.12M | 97.270 | 1.03M | 199.288 | 2.23× | 2.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.233 | 2.56× |
| 1 | 5 | 0.201 | 0.841 | 4.18× |
| 1 | 10 | 0.386 | 1.994 | 5.16× |
| 10 | 1 | 0.050 | 0.169 | 3.41× |
| 10 | 5 | 0.204 | 0.844 | 4.14× |
| 10 | 10 | 0.419 | 2.050 | 4.89× |
| 100 | 1 | 0.093 | 0.293 | 3.16× |
| 100 | 5 | 0.219 | 1.481 | 6.75× |
| 100 | 10 | 0.496 | 3.223 | 6.50× |
| 1,000 | 1 | 0.949 | 8.680 | 9.15× |
| 1,000 | 5 | 1.173 | 11.038 | 9.41× |
| 1,000 | 10 | 1.653 | 22.095 | 13.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
