# MathLog1p benchmark (`numpy.log1p` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.33M | 0.008 | 118.27M | 0.020 | 2.13× | 2.37× |
| 10,000 | 0.083 | 120.17M | 0.075 | 132.73M | 0.087 | 1.05× | 1.16× |
| 100,000 | 0.783 | 127.68M | 0.737 | 135.61M | 0.760 | 0.97× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.074 | 0.81× |
| 1 | 5 | 0.274 | 0.466 | 1.70× |
| 1 | 10 | 0.434 | 0.568 | 1.31× |
| 10 | 1 | 0.043 | 0.056 | 1.31× |
| 10 | 5 | 0.174 | 0.268 | 1.54× |
| 10 | 10 | 0.370 | 0.569 | 1.54× |
| 100 | 1 | 0.041 | 0.057 | 1.39× |
| 100 | 5 | 0.188 | 0.274 | 1.45× |
| 100 | 10 | 0.378 | 0.586 | 1.55× |
| 1,000 | 1 | 0.048 | 0.065 | 1.35× |
| 1,000 | 5 | 0.196 | 0.376 | 1.92× |
| 1,000 | 10 | 0.404 | 0.738 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
