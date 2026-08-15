# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.24M | 0.009 | 110.12M | 0.034 | 2.73× | 3.74× |
| 10,000 | 0.147 | 67.98M | 0.138 | 72.33M | 0.130 | 0.88× | 0.94× |
| 100,000 | 1.563 | 63.97M | 1.478 | 67.67M | 0.999 | 0.64× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.106 | 1.49× |
| 1 | 5 | 0.285 | 0.439 | 1.54× |
| 1 | 10 | 0.379 | 0.927 | 2.44× |
| 10 | 1 | 0.045 | 0.098 | 2.15× |
| 10 | 5 | 0.201 | 0.449 | 2.23× |
| 10 | 10 | 0.392 | 0.934 | 2.38× |
| 100 | 1 | 0.043 | 0.088 | 2.08× |
| 100 | 5 | 0.190 | 0.456 | 2.40× |
| 100 | 10 | 0.425 | 0.923 | 2.17× |
| 1,000 | 1 | 0.054 | 0.103 | 1.89× |
| 1,000 | 5 | 0.199 | 0.486 | 2.44× |
| 1,000 | 10 | 0.456 | 1.091 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
