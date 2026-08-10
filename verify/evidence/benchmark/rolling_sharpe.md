# RollingSharpe benchmark (`SharpeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.47M | 0.031 | 32.49M | 0.167 | 5.41× | 5.41× |
| 10,000 | 0.276 | 36.22M | 0.283 | 35.39M | 0.534 | 1.93× | 1.89× |
| 100,000 | 2.784 | 35.91M | 2.741 | 36.48M | 4.051 | 1.45× | 1.48× |
| 1,000,000 | 27.730 | 36.06M | 37.109 | 26.95M | 39.829 | 1.44× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.174 | 0.355 | 2.05× |
| 1 | 5 | 0.286 | 1.382 | 4.84× |
| 1 | 10 | 0.502 | 2.345 | 4.67× |
| 10 | 1 | 0.055 | 0.226 | 4.14× |
| 10 | 5 | 0.244 | 1.325 | 5.42× |
| 10 | 10 | 0.502 | 2.349 | 4.68× |
| 100 | 1 | 0.053 | 0.223 | 4.24× |
| 100 | 5 | 0.242 | 1.275 | 5.27× |
| 100 | 10 | 0.494 | 2.350 | 4.76× |
| 1,000 | 1 | 0.080 | 0.261 | 3.28× |
| 1,000 | 5 | 0.246 | 1.487 | 6.06× |
| 1,000 | 10 | 0.530 | 2.735 | 5.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
