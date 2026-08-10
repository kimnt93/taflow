# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.42M | 0.005 | 192.63M | 0.054 | 7.84× | 10.39× |
| 10,000 | 0.034 | 290.44M | 0.026 | 382.17M | 0.138 | 4.01× | 5.27× |
| 100,000 | 0.323 | 309.19M | 0.224 | 446.59M | 1.007 | 3.11× | 4.50× |
| 1,000,000 | 12.972 | 77.09M | 2.576 | 388.23M | 11.054 | 0.85× | 4.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.301 | 2.54× |
| 1 | 5 | 0.340 | 0.563 | 1.66× |
| 1 | 10 | 0.478 | 1.075 | 2.25× |
| 10 | 1 | 0.047 | 0.102 | 2.18× |
| 10 | 5 | 0.235 | 0.508 | 2.16× |
| 10 | 10 | 0.463 | 1.047 | 2.26× |
| 100 | 1 | 0.048 | 0.111 | 2.29× |
| 100 | 5 | 0.222 | 0.498 | 2.25× |
| 100 | 10 | 0.540 | 1.142 | 2.12× |
| 1,000 | 1 | 0.062 | 0.126 | 2.02× |
| 1,000 | 5 | 0.237 | 0.585 | 2.47× |
| 1,000 | 10 | 0.505 | 1.229 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
