# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.50M | 0.010 | 101.33M | 0.045 | 5.59× | 4.55× |
| 10,000 | 0.074 | 134.61M | 0.068 | 146.42M | 0.089 | 1.19× | 1.30× |
| 100,000 | 0.467 | 214.06M | 0.448 | 223.42M | 0.580 | 1.24× | 1.30× |
| 1,000,000 | 4.872 | 205.27M | 4.397 | 227.44M | 5.698 | 1.17× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.157 | 1.79× |
| 1 | 5 | 0.286 | 0.485 | 1.70× |
| 1 | 10 | 0.522 | 1.111 | 2.13× |
| 10 | 1 | 0.054 | 0.093 | 1.72× |
| 10 | 5 | 0.259 | 0.532 | 2.05× |
| 10 | 10 | 0.529 | 1.123 | 2.12× |
| 100 | 1 | 0.065 | 0.116 | 1.80× |
| 100 | 5 | 0.281 | 0.602 | 2.14× |
| 100 | 10 | 0.578 | 1.188 | 2.06× |
| 1,000 | 1 | 0.070 | 0.118 | 1.68× |
| 1,000 | 5 | 0.331 | 0.655 | 1.98× |
| 1,000 | 10 | 0.607 | 1.271 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
