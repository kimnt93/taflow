# Liquidity benchmark (`causal liquidity pools` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.55M | 0.036 | 27.48M | 4.705 | 129.62× | 129.32× |
| 10,000 | 0.400 | 25.03M | 0.383 | 26.08M | 70.169 | 175.60× | 183.00× |
| 100,000 | 4.263 | 23.46M | 4.507 | 22.19M | 1147.337 | 269.14× | 254.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.208 | 0.176 | 0.85× |
| 1 | 5 | 0.210 | 0.618 | 2.95× |
| 1 | 10 | 0.487 | 1.255 | 2.58× |
| 10 | 1 | 0.048 | 0.127 | 2.64× |
| 10 | 5 | 0.214 | 0.604 | 2.83× |
| 10 | 10 | 0.415 | 1.313 | 3.17× |
| 100 | 1 | 0.058 | 0.224 | 3.89× |
| 100 | 5 | 0.202 | 0.990 | 4.89× |
| 100 | 10 | 0.451 | 2.076 | 4.60× |
| 1,000 | 1 | 0.089 | 4.952 | 55.87× |
| 1,000 | 5 | 0.249 | 25.230 | 101.22× |
| 1,000 | 10 | 0.557 | 52.632 | 94.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
