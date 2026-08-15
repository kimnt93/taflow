# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 434.30M | 0.001 | 744.38M | 0.032 | 13.93× | 23.87× |
| 10,000 | 0.008 | 1.22G | 0.006 | 1.71G | 0.040 | 4.89× | 6.82× |
| 100,000 | 0.073 | 1.38G | 0.049 | 2.04G | 0.128 | 1.76× | 2.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.180 | 1.82× |
| 1 | 5 | 0.309 | 0.463 | 1.50× |
| 1 | 10 | 0.373 | 0.985 | 2.64× |
| 10 | 1 | 0.046 | 0.093 | 2.03× |
| 10 | 5 | 0.188 | 0.441 | 2.34× |
| 10 | 10 | 0.386 | 0.891 | 2.31× |
| 100 | 1 | 0.042 | 0.089 | 2.11× |
| 100 | 5 | 0.201 | 0.484 | 2.41× |
| 100 | 10 | 0.412 | 0.940 | 2.28× |
| 1,000 | 1 | 0.049 | 0.087 | 1.79× |
| 1,000 | 5 | 0.181 | 0.465 | 2.56× |
| 1,000 | 10 | 0.418 | 0.930 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
