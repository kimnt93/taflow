# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.83M | 0.008 | 126.53M | 0.038 | 4.24× | 4.76× |
| 10,000 | 0.057 | 176.81M | 0.054 | 184.92M | 0.090 | 1.59× | 1.66× |
| 100,000 | 0.542 | 184.61M | 0.580 | 172.36M | 0.599 | 1.11× | 1.03× |
| 1,000,000 | 6.021 | 166.10M | 5.404 | 185.06M | 6.684 | 1.11× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.142 | 1.47× |
| 1 | 5 | 0.275 | 0.481 | 1.75× |
| 1 | 10 | 0.452 | 1.112 | 2.46× |
| 10 | 1 | 0.053 | 0.086 | 1.60× |
| 10 | 5 | 0.235 | 0.472 | 2.01× |
| 10 | 10 | 0.452 | 0.894 | 1.98× |
| 100 | 1 | 0.047 | 0.090 | 1.90× |
| 100 | 5 | 0.210 | 0.431 | 2.05× |
| 100 | 10 | 0.445 | 0.926 | 2.08× |
| 1,000 | 1 | 0.053 | 0.100 | 1.89× |
| 1,000 | 5 | 0.256 | 0.476 | 1.86× |
| 1,000 | 10 | 0.481 | 0.957 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
