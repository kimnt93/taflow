# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.81M | 0.005 | 192.14M | 0.041 | 5.94× | 7.88× |
| 10,000 | 0.023 | 441.46M | 0.019 | 523.60M | 0.036 | 1.59× | 1.89× |
| 100,000 | 0.192 | 521.86M | 0.155 | 643.32M | 0.095 | 0.50× | 0.61× |
| 1,000,000 | 2.817 | 355.03M | 2.680 | 373.07M | 1.518 | 0.54× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.108 | 1.39× |
| 1 | 5 | 0.258 | 0.455 | 1.76× |
| 1 | 10 | 0.584 | 0.951 | 1.63× |
| 10 | 1 | 0.052 | 0.090 | 1.72× |
| 10 | 5 | 0.238 | 0.423 | 1.77× |
| 10 | 10 | 0.497 | 1.061 | 2.13× |
| 100 | 1 | 0.060 | 0.090 | 1.50× |
| 100 | 5 | 0.232 | 0.434 | 1.87× |
| 100 | 10 | 0.463 | 0.994 | 2.14× |
| 1,000 | 1 | 0.069 | 0.121 | 1.76× |
| 1,000 | 5 | 0.307 | 0.467 | 1.52× |
| 1,000 | 10 | 0.506 | 0.948 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
