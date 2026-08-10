# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.73M | 0.011 | 89.34M | 0.041 | 3.58× | 3.64× |
| 10,000 | 0.089 | 112.18M | 0.085 | 117.86M | 0.097 | 1.09× | 1.14× |
| 100,000 | 0.819 | 122.17M | 0.840 | 119.11M | 0.644 | 0.79× | 0.77× |
| 1,000,000 | 8.951 | 111.72M | 8.227 | 121.55M | 6.251 | 0.70× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.123 | 0.89× |
| 1 | 5 | 0.281 | 0.513 | 1.83× |
| 1 | 10 | 0.504 | 1.223 | 2.42× |
| 10 | 1 | 0.055 | 0.103 | 1.86× |
| 10 | 5 | 0.252 | 0.505 | 2.00× |
| 10 | 10 | 0.458 | 1.020 | 2.23× |
| 100 | 1 | 0.059 | 0.106 | 1.79× |
| 100 | 5 | 0.281 | 0.503 | 1.79× |
| 100 | 10 | 0.487 | 0.960 | 1.97× |
| 1,000 | 1 | 0.059 | 0.114 | 1.93× |
| 1,000 | 5 | 0.288 | 0.576 | 2.00× |
| 1,000 | 10 | 0.552 | 1.016 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
