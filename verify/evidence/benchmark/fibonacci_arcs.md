# FibonacciArcs benchmark (`FibArcs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.60M | 0.018 | 55.59M | 0.560 | 27.77× | 31.12× |
| 10,000 | 0.162 | 61.85M | 0.156 | 64.07M | 3.834 | 23.71× | 24.56× |
| 100,000 | 1.662 | 60.17M | 1.722 | 58.06M | 44.280 | 26.65× | 25.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.210 | 2.32× |
| 1 | 5 | 0.239 | 0.904 | 3.79× |
| 1 | 10 | 0.400 | 1.855 | 4.64× |
| 10 | 1 | 0.046 | 0.169 | 3.71× |
| 10 | 5 | 0.197 | 0.908 | 4.61× |
| 10 | 10 | 0.399 | 1.997 | 5.01× |
| 100 | 1 | 0.054 | 0.224 | 4.11× |
| 100 | 5 | 0.250 | 1.093 | 4.36× |
| 100 | 10 | 0.445 | 2.408 | 5.41× |
| 1,000 | 1 | 0.081 | 0.850 | 10.52× |
| 1,000 | 5 | 0.237 | 3.256 | 13.74× |
| 1,000 | 10 | 0.481 | 19.552 | 40.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
