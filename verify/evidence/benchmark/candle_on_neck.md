# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.52M | 0.003 | 337.77M | 0.033 | 3.76× | 10.99× |
| 10,000 | 0.063 | 158.08M | 0.052 | 191.46M | 0.125 | 1.97× | 2.39× |
| 100,000 | 0.880 | 113.67M | 0.779 | 128.43M | 0.914 | 1.04× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.103 | 0.90× |
| 1 | 5 | 0.259 | 0.452 | 1.74× |
| 1 | 10 | 0.393 | 0.898 | 2.29× |
| 10 | 1 | 0.061 | 0.113 | 1.86× |
| 10 | 5 | 0.183 | 0.507 | 2.77× |
| 10 | 10 | 0.410 | 0.928 | 2.27× |
| 100 | 1 | 0.047 | 0.089 | 1.89× |
| 100 | 5 | 0.188 | 0.426 | 2.26× |
| 100 | 10 | 0.419 | 0.974 | 2.32× |
| 1,000 | 1 | 0.049 | 0.098 | 2.00× |
| 1,000 | 5 | 0.190 | 0.461 | 2.43× |
| 1,000 | 10 | 0.412 | 1.026 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
