# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.97M | 0.006 | 157.13M | 0.033 | 4.58× | 5.18× |
| 10,000 | 0.049 | 202.43M | 0.043 | 231.22M | 0.060 | 1.21× | 1.39× |
| 100,000 | 0.408 | 245.25M | 0.392 | 254.92M | 0.320 | 0.78× | 0.82× |
| 1,000,000 | 4.665 | 214.38M | 3.878 | 257.86M | 2.934 | 0.63× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.117 | 1.34× |
| 1 | 5 | 0.276 | 0.451 | 1.63× |
| 1 | 10 | 0.485 | 0.981 | 2.02× |
| 10 | 1 | 0.046 | 0.094 | 2.06× |
| 10 | 5 | 0.229 | 0.444 | 1.94× |
| 10 | 10 | 0.510 | 0.941 | 1.85× |
| 100 | 1 | 0.050 | 0.086 | 1.70× |
| 100 | 5 | 0.240 | 0.488 | 2.03× |
| 100 | 10 | 0.460 | 0.906 | 1.97× |
| 1,000 | 1 | 0.053 | 0.098 | 1.85× |
| 1,000 | 5 | 0.219 | 0.447 | 2.04× |
| 1,000 | 10 | 0.494 | 1.170 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
