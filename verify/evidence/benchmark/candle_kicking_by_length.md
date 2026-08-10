# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.42M | 0.019 | 52.99M | 0.043 | 2.01× | 2.30× |
| 10,000 | 0.175 | 57.00M | 0.172 | 57.98M | 0.191 | 1.09× | 1.11× |
| 100,000 | 1.757 | 56.90M | 1.822 | 54.89M | 1.666 | 0.95× | 0.91× |
| 1,000,000 | 17.550 | 56.98M | 18.165 | 55.05M | 16.343 | 0.93× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.133 | 0.86× |
| 1 | 5 | 0.369 | 0.502 | 1.36× |
| 1 | 10 | 0.566 | 0.904 | 1.60× |
| 10 | 1 | 0.053 | 0.090 | 1.70× |
| 10 | 5 | 0.252 | 0.445 | 1.77× |
| 10 | 10 | 0.575 | 0.927 | 1.61× |
| 100 | 1 | 0.061 | 0.096 | 1.56× |
| 100 | 5 | 0.264 | 0.452 | 1.72× |
| 100 | 10 | 0.587 | 0.995 | 1.69× |
| 1,000 | 1 | 0.073 | 0.110 | 1.52× |
| 1,000 | 5 | 0.272 | 0.525 | 1.93× |
| 1,000 | 10 | 0.590 | 1.141 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
