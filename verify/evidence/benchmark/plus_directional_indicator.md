# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.43M | 0.013 | 76.22M | 0.050 | 3.12× | 3.81× |
| 10,000 | 0.084 | 118.77M | 0.079 | 126.15M | 0.121 | 1.44× | 1.53× |
| 100,000 | 0.818 | 122.29M | 0.693 | 144.22M | 0.862 | 1.05× | 1.24× |
| 1,000,000 | 8.397 | 119.09M | 7.415 | 134.86M | 8.168 | 0.97× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.148 | 1.90× |
| 1 | 5 | 0.332 | 0.622 | 1.87× |
| 1 | 10 | 0.591 | 1.176 | 1.99× |
| 10 | 1 | 0.071 | 0.134 | 1.89× |
| 10 | 5 | 0.320 | 0.606 | 1.90× |
| 10 | 10 | 0.557 | 1.079 | 1.94× |
| 100 | 1 | 0.068 | 0.117 | 1.72× |
| 100 | 5 | 0.364 | 0.587 | 1.61× |
| 100 | 10 | 0.564 | 1.000 | 1.77× |
| 1,000 | 1 | 0.063 | 0.112 | 1.78× |
| 1,000 | 5 | 0.327 | 0.637 | 1.95× |
| 1,000 | 10 | 0.682 | 1.088 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
