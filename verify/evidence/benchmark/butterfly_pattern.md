# ButterflyPattern benchmark (`Butterfly` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.31M | 0.007 | 134.48M | 0.222 | 18.97× | 29.90× |
| 10,000 | 0.101 | 99.41M | 0.090 | 110.53M | 1.333 | 13.25× | 14.73× |
| 100,000 | 0.932 | 107.30M | 0.896 | 111.57M | 13.434 | 14.41× | 14.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.273 | 1.89× |
| 1 | 5 | 0.265 | 0.850 | 3.21× |
| 1 | 10 | 0.418 | 1.623 | 3.89× |
| 10 | 1 | 0.046 | 0.162 | 3.54× |
| 10 | 5 | 0.189 | 1.181 | 6.26× |
| 10 | 10 | 0.407 | 1.649 | 4.05× |
| 100 | 1 | 0.044 | 0.180 | 4.09× |
| 100 | 5 | 0.214 | 1.193 | 5.58× |
| 100 | 10 | 0.410 | 1.790 | 4.37× |
| 1,000 | 1 | 0.063 | 0.299 | 4.73× |
| 1,000 | 5 | 0.227 | 1.818 | 8.00× |
| 1,000 | 10 | 0.414 | 3.044 | 7.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
