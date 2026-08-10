# FractalDimension benchmark (`two-chunk rescaled-range dimension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.230 | 4.34M | 0.215 | 4.64M | 0.773 | 3.35× | 3.59× |
| 10,000 | 2.182 | 4.58M | 2.151 | 4.65M | 5.325 | 2.44× | 2.48× |
| 100,000 | 21.391 | 4.67M | 21.380 | 4.68M | 61.634 | 2.88× | 2.88× |
| 1,000,000 | 216.758 | 4.61M | 214.044 | 4.67M | 633.331 | 2.92× | 2.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.111 | 1.23× |
| 1 | 5 | 0.285 | 0.443 | 1.56× |
| 1 | 10 | 0.488 | 0.896 | 1.83× |
| 10 | 1 | 0.050 | 0.085 | 1.69× |
| 10 | 5 | 0.221 | 0.430 | 1.94× |
| 10 | 10 | 0.475 | 0.899 | 1.89× |
| 100 | 1 | 0.069 | 0.371 | 5.40× |
| 100 | 5 | 0.251 | 2.257 | 8.99× |
| 100 | 10 | 0.549 | 4.323 | 7.88× |
| 1,000 | 1 | 0.291 | 0.884 | 3.04× |
| 1,000 | 5 | 0.488 | 3.236 | 6.63× |
| 1,000 | 10 | 0.735 | 6.658 | 9.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
