# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.96M | 0.019 | 52.42M | 0.050 | 2.73× | 2.61× |
| 10,000 | 0.155 | 64.53M | 0.158 | 63.44M | 0.184 | 1.19× | 1.17× |
| 100,000 | 1.507 | 66.36M | 1.393 | 71.79M | 1.506 | 1.00× | 1.08× |
| 1,000,000 | 18.287 | 54.68M | 13.940 | 71.74M | 14.093 | 0.77× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.129 | 1.09× |
| 1 | 5 | 0.317 | 0.491 | 1.55× |
| 1 | 10 | 0.542 | 3.740 | 6.90× |
| 10 | 1 | 0.074 | 0.120 | 1.63× |
| 10 | 5 | 0.328 | 0.683 | 2.08× |
| 10 | 10 | 0.656 | 1.577 | 2.40× |
| 100 | 1 | 0.070 | 0.168 | 2.41× |
| 100 | 5 | 0.415 | 0.879 | 2.12× |
| 100 | 10 | 0.879 | 1.336 | 1.52× |
| 1,000 | 1 | 0.065 | 0.110 | 1.68× |
| 1,000 | 5 | 0.310 | 0.580 | 1.87× |
| 1,000 | 10 | 0.665 | 1.402 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
