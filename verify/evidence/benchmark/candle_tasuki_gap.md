# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.55M | 0.018 | 56.73M | 0.043 | 2.00× | 2.44× |
| 10,000 | 0.161 | 62.08M | 0.159 | 62.83M | 0.188 | 1.16× | 1.18× |
| 100,000 | 1.606 | 62.25M | 1.642 | 60.92M | 1.545 | 0.96× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.176 | 0.105 | 0.60× |
| 1 | 5 | 0.326 | 0.504 | 1.55× |
| 1 | 10 | 0.578 | 0.935 | 1.62× |
| 10 | 1 | 0.055 | 0.088 | 1.62× |
| 10 | 5 | 0.284 | 0.443 | 1.56× |
| 10 | 10 | 0.516 | 0.998 | 1.93× |
| 100 | 1 | 0.059 | 0.110 | 1.88× |
| 100 | 5 | 0.275 | 0.462 | 1.68× |
| 100 | 10 | 0.587 | 0.989 | 1.69× |
| 1,000 | 1 | 0.077 | 0.106 | 1.37× |
| 1,000 | 5 | 0.287 | 0.530 | 1.85× |
| 1,000 | 10 | 0.552 | 1.095 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
