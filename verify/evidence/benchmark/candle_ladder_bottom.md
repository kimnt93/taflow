# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.35M | 0.003 | 369.94M | 0.034 | 5.88× | 12.62× |
| 10,000 | 0.050 | 199.28M | 0.045 | 221.94M | 0.089 | 1.78× | 1.98× |
| 100,000 | 0.570 | 175.54M | 0.572 | 174.78M | 0.647 | 1.14× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.129 | 1.23× |
| 1 | 5 | 0.214 | 0.473 | 2.20× |
| 1 | 10 | 0.390 | 1.011 | 2.59× |
| 10 | 1 | 0.048 | 0.120 | 2.53× |
| 10 | 5 | 0.253 | 0.506 | 2.00× |
| 10 | 10 | 0.458 | 0.946 | 2.07× |
| 100 | 1 | 0.044 | 0.090 | 2.03× |
| 100 | 5 | 0.189 | 0.629 | 3.33× |
| 100 | 10 | 0.463 | 0.960 | 2.07× |
| 1,000 | 1 | 0.048 | 0.092 | 1.93× |
| 1,000 | 5 | 0.224 | 0.500 | 2.24× |
| 1,000 | 10 | 0.561 | 1.132 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
