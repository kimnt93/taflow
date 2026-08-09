# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.39M | 0.005 | 191.29M | 0.052 | 7.85× | 9.99× |
| 10,000 | 0.034 | 290.31M | 0.027 | 374.48M | 0.142 | 4.13× | 5.33× |
| 100,000 | 0.315 | 317.76M | 0.243 | 411.05M | 1.038 | 3.30× | 4.27× |
| 1,000,000 | 13.382 | 74.73M | 2.489 | 401.69M | 10.901 | 0.81× | 4.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.154 | 1.67× |
| 1 | 5 | 0.391 | 0.559 | 1.43× |
| 1 | 10 | 0.490 | 1.118 | 2.28× |
| 10 | 1 | 0.048 | 0.106 | 2.22× |
| 10 | 5 | 0.225 | 0.547 | 2.44× |
| 10 | 10 | 0.506 | 1.129 | 2.23× |
| 100 | 1 | 0.048 | 0.106 | 2.19× |
| 100 | 5 | 0.258 | 0.567 | 2.20× |
| 100 | 10 | 0.510 | 1.146 | 2.25× |
| 1,000 | 1 | 0.056 | 0.115 | 2.07× |
| 1,000 | 5 | 0.243 | 0.600 | 2.47× |
| 1,000 | 10 | 0.513 | 1.216 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
