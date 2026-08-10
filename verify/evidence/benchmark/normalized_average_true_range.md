# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.31M | 0.012 | 80.64M | 0.050 | 3.48× | 4.00× |
| 10,000 | 0.147 | 68.10M | 0.095 | 105.21M | 0.095 | 0.65× | 1.00× |
| 100,000 | 0.849 | 117.84M | 0.743 | 134.60M | 0.665 | 0.78× | 0.90× |
| 1,000,000 | 8.896 | 112.42M | 8.423 | 118.73M | 7.297 | 0.82× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.129 | 1.27× |
| 1 | 5 | 0.262 | 0.558 | 2.13× |
| 1 | 10 | 0.502 | 1.097 | 2.18× |
| 10 | 1 | 0.072 | 0.102 | 1.42× |
| 10 | 5 | 0.270 | 0.534 | 1.98× |
| 10 | 10 | 0.558 | 1.054 | 1.89× |
| 100 | 1 | 0.055 | 0.097 | 1.77× |
| 100 | 5 | 0.264 | 0.494 | 1.87× |
| 100 | 10 | 0.514 | 1.007 | 1.96× |
| 1,000 | 1 | 0.058 | 0.097 | 1.68× |
| 1,000 | 5 | 0.283 | 0.526 | 1.86× |
| 1,000 | 10 | 0.582 | 1.092 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
