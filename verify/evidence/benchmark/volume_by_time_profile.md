# VolumeByTimeProfile benchmark (`VolumeByTimeProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.39M | 0.051 | 19.61M | 1.744 | 28.58× | 34.20× |
| 10,000 | 0.550 | 18.19M | 0.458 | 21.83M | 14.985 | 27.26× | 32.71× |
| 100,000 | 5.439 | 18.38M | 4.353 | 22.97M | 194.595 | 35.78× | 44.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.474 | 6.22× |
| 1 | 5 | 0.296 | 1.421 | 4.80× |
| 1 | 10 | 0.403 | 2.819 | 7.00× |
| 10 | 1 | 0.045 | 0.266 | 5.92× |
| 10 | 5 | 0.215 | 1.567 | 7.27× |
| 10 | 10 | 0.446 | 3.003 | 6.73× |
| 100 | 1 | 0.053 | 0.400 | 7.60× |
| 100 | 5 | 0.215 | 2.218 | 10.29× |
| 100 | 10 | 0.437 | 4.290 | 9.83× |
| 1,000 | 1 | 0.097 | 1.986 | 20.54× |
| 1,000 | 5 | 0.253 | 9.444 | 37.32× |
| 1,000 | 10 | 0.535 | 19.718 | 36.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
