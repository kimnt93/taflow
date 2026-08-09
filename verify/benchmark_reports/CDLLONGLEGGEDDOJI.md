# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.75M | 0.009 | 116.17M | 0.037 | 3.10× | 4.25× |
| 10,000 | 0.057 | 175.52M | 0.052 | 190.84M | 0.092 | 1.62× | 1.76× |
| 100,000 | 0.564 | 177.39M | 0.538 | 186.04M | 0.678 | 1.20× | 1.26× |
| 1,000,000 | 5.687 | 175.84M | 5.480 | 182.48M | 6.556 | 1.15× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.112 | 1.14× |
| 1 | 5 | 0.344 | 0.488 | 1.42× |
| 1 | 10 | 0.553 | 0.963 | 1.74× |
| 10 | 1 | 0.053 | 0.095 | 1.79× |
| 10 | 5 | 0.250 | 0.458 | 1.83× |
| 10 | 10 | 0.505 | 0.921 | 1.82× |
| 100 | 1 | 0.053 | 0.093 | 1.78× |
| 100 | 5 | 0.250 | 0.449 | 1.79× |
| 100 | 10 | 0.514 | 0.934 | 1.82× |
| 1,000 | 1 | 0.063 | 0.109 | 1.74× |
| 1,000 | 5 | 0.249 | 0.466 | 1.87× |
| 1,000 | 10 | 0.533 | 1.014 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
