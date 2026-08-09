# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.60M | 0.008 | 129.24M | 0.043 | 4.88× | 5.55× |
| 10,000 | 0.049 | 205.29M | 0.046 | 219.52M | 0.089 | 1.84× | 1.96× |
| 100,000 | 0.449 | 222.48M | 0.423 | 236.16M | 0.560 | 1.25× | 1.32× |
| 1,000,000 | 4.700 | 212.77M | 4.456 | 224.44M | 5.411 | 1.15× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.187 | 1.13× |
| 1 | 5 | 0.268 | 0.534 | 2.00× |
| 1 | 10 | 0.547 | 1.043 | 1.91× |
| 10 | 1 | 0.059 | 0.117 | 2.00× |
| 10 | 5 | 0.249 | 0.510 | 2.05× |
| 10 | 10 | 0.561 | 1.146 | 2.04× |
| 100 | 1 | 0.071 | 0.114 | 1.60× |
| 100 | 5 | 0.255 | 0.507 | 1.99× |
| 100 | 10 | 0.523 | 1.008 | 1.93× |
| 1,000 | 1 | 0.057 | 0.101 | 1.77× |
| 1,000 | 5 | 0.266 | 0.541 | 2.04× |
| 1,000 | 10 | 0.574 | 1.042 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
