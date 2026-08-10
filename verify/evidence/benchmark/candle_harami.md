# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.09M | 0.021 | 48.62M | 0.042 | 1.71× | 2.02× |
| 10,000 | 0.171 | 58.44M | 0.168 | 59.39M | 0.163 | 0.95× | 0.97× |
| 100,000 | 1.634 | 61.20M | 1.605 | 62.30M | 1.339 | 0.82× | 0.83× |
| 1,000,000 | 16.525 | 60.51M | 16.933 | 59.06M | 13.206 | 0.80× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.152 | 1.60× |
| 1 | 5 | 0.334 | 0.547 | 1.64× |
| 1 | 10 | 0.548 | 1.003 | 1.83× |
| 10 | 1 | 0.066 | 0.110 | 1.66× |
| 10 | 5 | 0.321 | 0.555 | 1.73× |
| 10 | 10 | 0.593 | 1.034 | 1.74× |
| 100 | 1 | 0.058 | 0.089 | 1.52× |
| 100 | 5 | 0.322 | 0.504 | 1.57× |
| 100 | 10 | 0.616 | 0.958 | 1.55× |
| 1,000 | 1 | 0.074 | 0.119 | 1.60× |
| 1,000 | 5 | 0.328 | 0.580 | 1.77× |
| 1,000 | 10 | 0.743 | 1.138 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
