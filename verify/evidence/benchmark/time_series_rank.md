# TimeSeriesRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.464 | 2.16M | 0.474 | 2.11M | 0.131 | 0.28× | 0.28× |
| 10,000 | 4.604 | 2.17M | 4.615 | 2.17M | 0.688 | 0.15× | 0.15× |
| 100,000 | 45.809 | 2.18M | 46.247 | 2.16M | 6.332 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.142 | 1.21× |
| 1 | 5 | 0.372 | 0.609 | 1.63× |
| 1 | 10 | 0.608 | 1.094 | 1.80× |
| 10 | 1 | 0.072 | 0.103 | 1.44× |
| 10 | 5 | 0.286 | 0.504 | 1.76× |
| 10 | 10 | 0.582 | 1.060 | 1.82× |
| 100 | 1 | 0.121 | 0.155 | 1.28× |
| 100 | 5 | 0.316 | 0.790 | 2.50× |
| 100 | 10 | 0.628 | 1.632 | 2.60× |
| 1,000 | 1 | 0.555 | 0.211 | 0.38× |
| 1,000 | 5 | 0.722 | 0.942 | 1.31× |
| 1,000 | 10 | 1.297 | 2.041 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
