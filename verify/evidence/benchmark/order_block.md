# OrderBlock benchmark (`causal dual-scale order blocks` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.63M | 0.081 | 12.28M | 9.580 | 111.41× | 117.66× |
| 10,000 | 0.876 | 11.41M | 0.867 | 11.53M | 119.886 | 136.79× | 138.26× |
| 100,000 | 9.395 | 10.64M | 9.627 | 10.39M | 1293.621 | 137.69× | 134.37× |
| 1,000,000 | 109.022 | 9.17M | 102.995 | 9.71M | 12741.316 | 116.87× | 123.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.216 | 2.00× |
| 1 | 5 | 0.420 | 0.920 | 2.19× |
| 1 | 10 | 0.566 | 1.770 | 3.13× |
| 10 | 1 | 0.061 | 0.195 | 3.18× |
| 10 | 5 | 0.295 | 0.957 | 3.24× |
| 10 | 10 | 0.572 | 1.933 | 3.38× |
| 100 | 1 | 0.071 | 0.645 | 9.12× |
| 100 | 5 | 0.281 | 3.253 | 11.57× |
| 100 | 10 | 0.588 | 6.447 | 10.96× |
| 1,000 | 1 | 0.144 | 9.491 | 65.69× |
| 1,000 | 5 | 0.494 | 82.802 | 167.62× |
| 1,000 | 10 | 0.868 | 109.312 | 125.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
