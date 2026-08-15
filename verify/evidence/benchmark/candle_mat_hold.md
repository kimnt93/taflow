# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.72M | 0.005 | 219.28M | 0.037 | 4.04× | 8.15× |
| 10,000 | 0.089 | 112.99M | 0.081 | 123.08M | 0.119 | 1.34× | 1.46× |
| 100,000 | 0.950 | 105.30M | 0.899 | 111.22M | 0.832 | 0.88× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.148 | 1.15× |
| 1 | 5 | 0.384 | 0.501 | 1.30× |
| 1 | 10 | 0.394 | 0.948 | 2.41× |
| 10 | 1 | 0.041 | 0.090 | 2.18× |
| 10 | 5 | 0.180 | 0.460 | 2.55× |
| 10 | 10 | 0.389 | 0.934 | 2.40× |
| 100 | 1 | 0.045 | 0.097 | 2.17× |
| 100 | 5 | 0.181 | 0.452 | 2.50× |
| 100 | 10 | 0.368 | 0.946 | 2.57× |
| 1,000 | 1 | 0.061 | 0.106 | 1.75× |
| 1,000 | 5 | 0.203 | 0.483 | 2.38× |
| 1,000 | 10 | 0.402 | 1.004 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
