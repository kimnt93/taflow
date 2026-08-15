# RollingInterquartileRange benchmark (`RollingIqr` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.10M | 0.050 | 20.07M | 0.318 | 6.07× | 6.38× |
| 10,000 | 0.539 | 18.57M | 0.527 | 18.99M | 1.772 | 3.29× | 3.37× |
| 100,000 | 5.509 | 18.15M | 5.355 | 18.68M | 16.685 | 3.03× | 3.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.280 | 4.09× |
| 1 | 5 | 0.233 | 1.092 | 4.70× |
| 1 | 10 | 0.392 | 2.672 | 6.81× |
| 10 | 1 | 0.046 | 0.214 | 4.67× |
| 10 | 5 | 0.188 | 1.038 | 5.53× |
| 10 | 10 | 0.438 | 2.419 | 5.52× |
| 100 | 1 | 0.053 | 0.231 | 4.35× |
| 100 | 5 | 0.203 | 1.519 | 7.48× |
| 100 | 10 | 0.461 | 2.519 | 5.47× |
| 1,000 | 1 | 0.106 | 0.393 | 3.72× |
| 1,000 | 5 | 0.248 | 2.322 | 9.36× |
| 1,000 | 10 | 0.459 | 4.303 | 9.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
