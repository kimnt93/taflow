# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.73M | 0.005 | 198.05M | 0.055 | 8.03× | 10.83× |
| 10,000 | 0.052 | 193.28M | 0.041 | 241.58M | 0.145 | 2.79× | 3.49× |
| 100,000 | 1.554 | 64.34M | 0.379 | 263.80M | 1.684 | 1.08× | 4.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.121 | 1.88× |
| 1 | 5 | 0.242 | 0.509 | 2.10× |
| 1 | 10 | 0.396 | 1.109 | 2.80× |
| 10 | 1 | 0.052 | 0.102 | 1.98× |
| 10 | 5 | 0.183 | 0.503 | 2.76× |
| 10 | 10 | 0.389 | 1.038 | 2.67× |
| 100 | 1 | 0.045 | 0.110 | 2.46× |
| 100 | 5 | 0.211 | 0.540 | 2.56× |
| 100 | 10 | 0.442 | 1.076 | 2.43× |
| 1,000 | 1 | 0.051 | 0.112 | 2.19× |
| 1,000 | 5 | 0.205 | 0.593 | 2.89× |
| 1,000 | 10 | 0.460 | 1.188 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
