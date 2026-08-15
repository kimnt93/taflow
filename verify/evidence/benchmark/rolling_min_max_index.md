# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.87M | 0.009 | 108.17M | 0.045 | 4.32× | 4.82× |
| 10,000 | 0.105 | 95.05M | 0.101 | 98.92M | 0.151 | 1.43× | 1.49× |
| 100,000 | 1.937 | 51.62M | 1.041 | 96.02M | 1.272 | 0.66× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.133 | 2.35× |
| 1 | 5 | 0.236 | 0.530 | 2.25× |
| 1 | 10 | 0.386 | 1.109 | 2.88× |
| 10 | 1 | 0.052 | 0.111 | 2.13× |
| 10 | 5 | 0.216 | 0.489 | 2.26× |
| 10 | 10 | 0.444 | 1.025 | 2.31× |
| 100 | 1 | 0.044 | 0.098 | 2.24× |
| 100 | 5 | 0.263 | 0.564 | 2.15× |
| 100 | 10 | 0.442 | 1.034 | 2.34× |
| 1,000 | 1 | 0.052 | 0.101 | 1.95× |
| 1,000 | 5 | 0.196 | 0.600 | 3.06× |
| 1,000 | 10 | 0.495 | 1.158 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
