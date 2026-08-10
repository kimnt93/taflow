# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.92M | 0.009 | 105.31M | 0.045 | 4.28× | 4.75× |
| 10,000 | 0.094 | 106.22M | 0.090 | 111.71M | 0.114 | 1.21× | 1.28× |
| 100,000 | 0.892 | 112.14M | 0.973 | 102.77M | 0.873 | 0.98× | 0.90× |
| 1,000,000 | 11.324 | 88.30M | 10.415 | 96.02M | 8.629 | 0.76× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.123 | 1.83× |
| 1 | 5 | 0.296 | 0.492 | 1.66× |
| 1 | 10 | 0.534 | 1.201 | 2.25× |
| 10 | 1 | 0.056 | 0.098 | 1.74× |
| 10 | 5 | 0.262 | 0.507 | 1.93× |
| 10 | 10 | 0.527 | 1.205 | 2.29× |
| 100 | 1 | 0.057 | 0.118 | 2.05× |
| 100 | 5 | 0.289 | 0.500 | 1.73× |
| 100 | 10 | 0.549 | 1.051 | 1.91× |
| 1,000 | 1 | 0.074 | 0.124 | 1.67× |
| 1,000 | 5 | 0.311 | 0.549 | 1.77× |
| 1,000 | 10 | 0.540 | 1.145 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
