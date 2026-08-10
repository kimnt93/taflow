# RollingAlpha benchmark (`Alpha` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.35M | 0.040 | 24.91M | 0.222 | 5.18× | 5.53× |
| 10,000 | 0.390 | 25.65M | 0.381 | 26.25M | 0.905 | 2.32× | 2.38× |
| 100,000 | 3.954 | 25.29M | 3.671 | 27.24M | 7.820 | 1.98× | 2.13× |
| 1,000,000 | 39.409 | 25.37M | 44.798 | 22.32M | 79.838 | 2.03× | 1.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.322 | 2.28× |
| 1 | 5 | 0.351 | 1.411 | 4.02× |
| 1 | 10 | 0.511 | 2.485 | 4.86× |
| 10 | 1 | 0.051 | 0.232 | 4.56× |
| 10 | 5 | 0.240 | 1.424 | 5.94× |
| 10 | 10 | 0.498 | 2.568 | 5.16× |
| 100 | 1 | 0.054 | 0.236 | 4.38× |
| 100 | 5 | 0.242 | 1.494 | 6.16× |
| 100 | 10 | 0.550 | 2.617 | 4.76× |
| 1,000 | 1 | 0.099 | 0.312 | 3.16× |
| 1,000 | 5 | 0.267 | 1.853 | 6.94× |
| 1,000 | 10 | 0.644 | 3.355 | 5.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
