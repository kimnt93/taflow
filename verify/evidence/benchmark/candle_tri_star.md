# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.32M | 0.010 | 103.14M | 0.036 | 2.83× | 3.68× |
| 10,000 | 0.107 | 93.67M | 0.106 | 94.35M | 0.095 | 0.89× | 0.90× |
| 100,000 | 1.072 | 93.30M | 1.064 | 94.02M | 0.618 | 0.58× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.131 | 1.52× |
| 1 | 5 | 0.244 | 0.458 | 1.88× |
| 1 | 10 | 0.404 | 0.926 | 2.29× |
| 10 | 1 | 0.051 | 0.103 | 2.04× |
| 10 | 5 | 0.178 | 0.436 | 2.46× |
| 10 | 10 | 0.394 | 0.910 | 2.31× |
| 100 | 1 | 0.045 | 0.089 | 1.98× |
| 100 | 5 | 0.184 | 0.466 | 2.54× |
| 100 | 10 | 0.400 | 0.928 | 2.32× |
| 1,000 | 1 | 0.055 | 0.100 | 1.82× |
| 1,000 | 5 | 0.209 | 0.476 | 2.27× |
| 1,000 | 10 | 0.402 | 1.041 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
