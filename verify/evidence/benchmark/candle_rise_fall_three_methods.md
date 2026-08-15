# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.30M | 0.004 | 226.93M | 0.035 | 4.37× | 7.86× |
| 10,000 | 0.098 | 101.69M | 0.095 | 105.01M | 0.113 | 1.15× | 1.19× |
| 100,000 | 1.171 | 85.37M | 1.143 | 87.49M | 0.942 | 0.80× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.157 | 1.55× |
| 1 | 5 | 0.223 | 0.441 | 1.98× |
| 1 | 10 | 0.415 | 0.912 | 2.20× |
| 10 | 1 | 0.039 | 0.096 | 2.43× |
| 10 | 5 | 0.222 | 0.482 | 2.17× |
| 10 | 10 | 0.383 | 0.944 | 2.46× |
| 100 | 1 | 0.043 | 0.091 | 2.11× |
| 100 | 5 | 0.188 | 0.462 | 2.46× |
| 100 | 10 | 0.436 | 0.972 | 2.23× |
| 1,000 | 1 | 0.054 | 0.103 | 1.91× |
| 1,000 | 5 | 0.213 | 0.482 | 2.26× |
| 1,000 | 10 | 0.432 | 1.110 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
