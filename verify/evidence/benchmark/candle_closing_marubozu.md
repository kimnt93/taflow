# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.088 | 11.40M | 0.084 | 11.86M | 0.035 | 0.40× | 0.42× |
| 10,000 | 0.696 | 14.37M | 0.725 | 13.80M | 0.122 | 0.17× | 0.17× |
| 100,000 | 6.685 | 14.96M | 7.176 | 13.94M | 0.942 | 0.14× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.217 | 0.116 | 0.53× |
| 1 | 5 | 0.410 | 0.506 | 1.24× |
| 1 | 10 | 0.676 | 0.908 | 1.34× |
| 10 | 1 | 0.067 | 0.094 | 1.40× |
| 10 | 5 | 0.305 | 0.404 | 1.33× |
| 10 | 10 | 0.612 | 0.899 | 1.47× |
| 100 | 1 | 0.076 | 0.092 | 1.20× |
| 100 | 5 | 0.301 | 0.415 | 1.38× |
| 100 | 10 | 0.661 | 0.900 | 1.36× |
| 1,000 | 1 | 0.142 | 0.095 | 0.67× |
| 1,000 | 5 | 0.330 | 0.492 | 1.49× |
| 1,000 | 10 | 0.676 | 0.980 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
