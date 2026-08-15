# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.53M | 0.006 | 180.22M | 0.039 | 4.59× | 7.09× |
| 10,000 | 0.095 | 104.94M | 0.095 | 105.06M | 0.177 | 1.86× | 1.86× |
| 100,000 | 1.139 | 87.77M | 1.097 | 91.14M | 1.402 | 1.23× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.107 | 1.34× |
| 1 | 5 | 0.255 | 0.436 | 1.71× |
| 1 | 10 | 0.387 | 0.892 | 2.31× |
| 10 | 1 | 0.041 | 0.086 | 2.12× |
| 10 | 5 | 0.182 | 0.420 | 2.30× |
| 10 | 10 | 0.355 | 0.861 | 2.43× |
| 100 | 1 | 0.052 | 0.090 | 1.74× |
| 100 | 5 | 0.188 | 0.459 | 2.44× |
| 100 | 10 | 0.405 | 0.889 | 2.19× |
| 1,000 | 1 | 0.058 | 0.097 | 1.69× |
| 1,000 | 5 | 0.203 | 0.493 | 2.44× |
| 1,000 | 10 | 0.415 | 1.041 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
