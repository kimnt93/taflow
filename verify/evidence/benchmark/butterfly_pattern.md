# ButterflyPattern benchmark (`Butterfly` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.22M | 0.011 | 89.33M | 0.217 | 15.05× | 19.42× |
| 10,000 | 0.095 | 105.02M | 0.090 | 111.27M | 1.344 | 14.12× | 14.96× |
| 100,000 | 0.929 | 107.63M | 0.860 | 116.29M | 12.018 | 12.94× | 13.98× |
| 1,000,000 | 9.094 | 109.97M | 8.840 | 113.12M | 122.814 | 13.51× | 13.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.222 | 2.12× |
| 1 | 5 | 0.276 | 1.075 | 3.90× |
| 1 | 10 | 0.573 | 1.804 | 3.15× |
| 10 | 1 | 0.060 | 0.171 | 2.85× |
| 10 | 5 | 0.245 | 1.080 | 4.41× |
| 10 | 10 | 0.523 | 1.690 | 3.23× |
| 100 | 1 | 0.057 | 0.186 | 3.27× |
| 100 | 5 | 0.263 | 1.143 | 4.35× |
| 100 | 10 | 0.538 | 1.750 | 3.25× |
| 1,000 | 1 | 0.074 | 0.297 | 4.02× |
| 1,000 | 5 | 0.253 | 1.749 | 6.90× |
| 1,000 | 10 | 0.565 | 3.041 | 5.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
