# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.79M | 0.004 | 222.47M | 0.017 | 3.05× | 3.72× |
| 10,000 | 0.036 | 274.78M | 0.031 | 321.04M | 0.027 | 0.75× | 0.88× |
| 100,000 | 0.317 | 315.48M | 0.292 | 342.52M | 0.132 | 0.42× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.094 | 0.98× |
| 1 | 5 | 0.299 | 0.338 | 1.13× |
| 1 | 10 | 0.403 | 0.636 | 1.58× |
| 10 | 1 | 0.043 | 0.066 | 1.54× |
| 10 | 5 | 0.177 | 0.314 | 1.78× |
| 10 | 10 | 0.417 | 0.675 | 1.62× |
| 100 | 1 | 0.044 | 0.065 | 1.48× |
| 100 | 5 | 0.184 | 0.309 | 1.68× |
| 100 | 10 | 0.402 | 0.681 | 1.70× |
| 1,000 | 1 | 0.049 | 0.067 | 1.37× |
| 1,000 | 5 | 0.192 | 0.388 | 2.02× |
| 1,000 | 10 | 0.415 | 1.017 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
