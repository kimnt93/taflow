# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.89M | 0.008 | 124.36M | 0.032 | 3.63× | 4.03× |
| 10,000 | 0.059 | 168.43M | 0.059 | 168.84M | 0.083 | 1.39× | 1.40× |
| 100,000 | 0.555 | 180.08M | 0.545 | 183.41M | 0.603 | 1.09× | 1.11× |
| 1,000,000 | 5.859 | 170.69M | 5.240 | 190.83M | 5.566 | 0.95× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.100 | 1.19× |
| 1 | 5 | 0.361 | 0.460 | 1.27× |
| 1 | 10 | 0.449 | 0.844 | 1.88× |
| 10 | 1 | 0.045 | 0.088 | 1.97× |
| 10 | 5 | 0.214 | 0.411 | 1.92× |
| 10 | 10 | 0.458 | 0.866 | 1.89× |
| 100 | 1 | 0.048 | 0.090 | 1.88× |
| 100 | 5 | 0.221 | 0.406 | 1.84× |
| 100 | 10 | 0.467 | 0.865 | 1.85× |
| 1,000 | 1 | 0.049 | 0.089 | 1.81× |
| 1,000 | 5 | 0.221 | 0.444 | 2.01× |
| 1,000 | 10 | 0.510 | 0.964 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
