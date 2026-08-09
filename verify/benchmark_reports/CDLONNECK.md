# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.02M | 0.008 | 118.74M | 0.033 | 3.51× | 3.89× |
| 10,000 | 0.071 | 140.66M | 0.065 | 152.91M | 0.122 | 1.72× | 1.87× |
| 100,000 | 0.898 | 111.40M | 0.884 | 113.16M | 0.927 | 1.03× | 1.05× |
| 1,000,000 | 9.766 | 102.40M | 9.093 | 109.98M | 9.380 | 0.96× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.110 | 1.11× |
| 1 | 5 | 0.471 | 0.610 | 1.30× |
| 1 | 10 | 0.539 | 0.953 | 1.77× |
| 10 | 1 | 0.055 | 0.092 | 1.68× |
| 10 | 5 | 0.238 | 0.426 | 1.79× |
| 10 | 10 | 0.507 | 0.925 | 1.83× |
| 100 | 1 | 0.053 | 0.088 | 1.66× |
| 100 | 5 | 0.255 | 0.453 | 1.77× |
| 100 | 10 | 0.545 | 0.941 | 1.73× |
| 1,000 | 1 | 0.067 | 0.102 | 1.51× |
| 1,000 | 5 | 0.250 | 0.492 | 1.97× |
| 1,000 | 10 | 0.567 | 1.039 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
