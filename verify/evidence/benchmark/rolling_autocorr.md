# RollingAutocorr benchmark (`Autocorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.19M | 0.065 | 15.44M | 0.270 | 4.10× | 4.17× |
| 10,000 | 0.625 | 15.99M | 0.641 | 15.59M | 1.127 | 1.80× | 1.76× |
| 100,000 | 6.115 | 16.35M | 6.084 | 16.44M | 9.926 | 1.62× | 1.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.408 | 5.66× |
| 1 | 5 | 0.312 | 1.422 | 4.56× |
| 1 | 10 | 0.468 | 2.877 | 6.14× |
| 10 | 1 | 0.054 | 0.216 | 4.01× |
| 10 | 5 | 0.246 | 1.503 | 6.11× |
| 10 | 10 | 0.510 | 2.588 | 5.07× |
| 100 | 1 | 0.062 | 0.252 | 4.04× |
| 100 | 5 | 0.262 | 1.607 | 6.14× |
| 100 | 10 | 0.518 | 2.945 | 5.69× |
| 1,000 | 1 | 0.115 | 0.339 | 2.94× |
| 1,000 | 5 | 0.270 | 2.004 | 7.41× |
| 1,000 | 10 | 0.561 | 3.640 | 6.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
