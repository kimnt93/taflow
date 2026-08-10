# FibonacciConfluence benchmark (`FibConfluence` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.453 | 2.21M | 0.476 | 2.10M | 1.628 | 3.59× | 3.42× |
| 10,000 | 5.064 | 1.97M | 5.057 | 1.98M | 17.401 | 3.44× | 3.44× |
| 100,000 | 51.210 | 1.95M | 51.081 | 1.96M | 174.220 | 3.40× | 3.41× |
| 1,000,000 | 510.616 | 1.96M | 511.352 | 1.96M | 1770.428 | 3.47× | 3.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.220 | 2.06× |
| 1 | 5 | 0.298 | 0.856 | 2.87× |
| 1 | 10 | 0.489 | 1.853 | 3.79× |
| 10 | 1 | 0.058 | 0.174 | 3.01× |
| 10 | 5 | 0.232 | 0.841 | 3.63× |
| 10 | 10 | 0.510 | 2.079 | 4.07× |
| 100 | 1 | 0.093 | 0.295 | 3.16× |
| 100 | 5 | 0.322 | 1.212 | 3.77× |
| 100 | 10 | 0.521 | 2.549 | 4.89× |
| 1,000 | 1 | 0.557 | 2.254 | 4.05× |
| 1,000 | 5 | 0.702 | 10.105 | 14.40× |
| 1,000 | 10 | 1.224 | 20.159 | 16.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
