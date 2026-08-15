# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.63M | 0.011 | 90.78M | 0.041 | 3.15× | 3.73× |
| 10,000 | 0.103 | 96.66M | 0.100 | 100.26M | 0.129 | 1.25× | 1.29× |
| 100,000 | 1.032 | 96.92M | 0.990 | 100.97M | 1.110 | 1.08× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.105 | 1.62× |
| 1 | 5 | 0.290 | 0.448 | 1.54× |
| 1 | 10 | 0.374 | 0.913 | 2.44× |
| 10 | 1 | 0.045 | 0.103 | 2.27× |
| 10 | 5 | 0.210 | 0.447 | 2.13× |
| 10 | 10 | 0.363 | 0.863 | 2.38× |
| 100 | 1 | 0.047 | 0.084 | 1.78× |
| 100 | 5 | 0.186 | 0.440 | 2.37× |
| 100 | 10 | 0.416 | 0.938 | 2.25× |
| 1,000 | 1 | 0.055 | 0.098 | 1.79× |
| 1,000 | 5 | 0.194 | 0.472 | 2.43× |
| 1,000 | 10 | 0.435 | 1.099 | 2.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
