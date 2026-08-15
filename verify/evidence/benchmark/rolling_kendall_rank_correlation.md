# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.69M | 0.023 | 43.17M | 0.784 | 32.67× | 33.82× |
| 10,000 | 0.224 | 44.67M | 0.225 | 44.39M | 6.781 | 30.29× | 30.10× |
| 100,000 | 2.286 | 43.75M | 2.311 | 43.28M | 66.732 | 29.19× | 28.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.241 | 3.29× |
| 1 | 5 | 0.322 | 1.280 | 3.98× |
| 1 | 10 | 0.393 | 2.188 | 5.57× |
| 10 | 1 | 0.041 | 0.221 | 5.43× |
| 10 | 5 | 0.171 | 1.214 | 7.08× |
| 10 | 10 | 0.373 | 2.160 | 5.78× |
| 100 | 1 | 0.049 | 0.284 | 5.80× |
| 100 | 5 | 0.187 | 1.503 | 8.04× |
| 100 | 10 | 0.402 | 2.784 | 6.93× |
| 1,000 | 1 | 0.067 | 0.923 | 13.84× |
| 1,000 | 5 | 0.188 | 4.761 | 25.37× |
| 1,000 | 10 | 0.408 | 9.339 | 22.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
