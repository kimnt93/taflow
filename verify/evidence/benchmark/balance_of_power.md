# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.25M | 0.007 | 136.38M | 0.029 | 3.01× | 3.90× |
| 10,000 | 0.041 | 241.35M | 0.029 | 339.45M | 0.040 | 0.98× | 1.37× |
| 100,000 | 0.280 | 356.55M | 0.243 | 411.87M | 0.148 | 0.53× | 0.61× |
| 1,000,000 | 3.648 | 274.09M | 3.036 | 329.38M | 2.390 | 0.66× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.115 | 1.30× |
| 1 | 5 | 0.266 | 0.514 | 1.93× |
| 1 | 10 | 0.767 | 1.160 | 1.51× |
| 10 | 1 | 0.057 | 0.093 | 1.63× |
| 10 | 5 | 0.315 | 0.507 | 1.61× |
| 10 | 10 | 0.733 | 0.989 | 1.35× |
| 100 | 1 | 0.059 | 0.121 | 2.05× |
| 100 | 5 | 0.294 | 0.464 | 1.57× |
| 100 | 10 | 0.686 | 1.043 | 1.52× |
| 1,000 | 1 | 0.084 | 0.101 | 1.20× |
| 1,000 | 5 | 0.280 | 0.466 | 1.66× |
| 1,000 | 10 | 0.574 | 1.146 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
