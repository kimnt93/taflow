# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.70M | 0.007 | 134.14M | 0.041 | 4.34× | 5.45× |
| 10,000 | 0.046 | 219.58M | 0.043 | 230.51M | 0.084 | 1.85× | 1.94× |
| 100,000 | 0.416 | 240.37M | 0.391 | 255.52M | 0.507 | 1.22× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.138 | 0.90× |
| 1 | 5 | 0.291 | 0.497 | 1.71× |
| 1 | 10 | 0.482 | 1.004 | 2.08× |
| 10 | 1 | 0.052 | 0.101 | 1.96× |
| 10 | 5 | 0.246 | 0.470 | 1.91× |
| 10 | 10 | 0.520 | 1.044 | 2.01× |
| 100 | 1 | 0.055 | 0.096 | 1.75× |
| 100 | 5 | 0.232 | 0.483 | 2.08× |
| 100 | 10 | 0.498 | 1.015 | 2.04× |
| 1,000 | 1 | 0.060 | 0.099 | 1.64× |
| 1,000 | 5 | 0.243 | 0.480 | 1.97× |
| 1,000 | 10 | 0.477 | 1.078 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
