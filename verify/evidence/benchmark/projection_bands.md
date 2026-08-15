# ProjectionBands benchmark (`rolling projection mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.99M | 0.016 | 61.25M | 0.082 | 4.90× | 5.00× |
| 10,000 | 0.157 | 63.66M | 0.155 | 64.47M | 0.282 | 1.79× | 1.82× |
| 100,000 | 1.504 | 66.48M | 1.493 | 66.97M | 2.300 | 1.53× | 1.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.126 | 1.70× |
| 1 | 5 | 0.233 | 0.522 | 2.24× |
| 1 | 10 | 0.412 | 1.046 | 2.54× |
| 10 | 1 | 0.041 | 0.099 | 2.43× |
| 10 | 5 | 0.183 | 0.486 | 2.65× |
| 10 | 10 | 0.394 | 1.087 | 2.76× |
| 100 | 1 | 0.050 | 0.141 | 2.84× |
| 100 | 5 | 0.187 | 0.705 | 3.78× |
| 100 | 10 | 0.424 | 1.447 | 3.41× |
| 1,000 | 1 | 0.076 | 0.173 | 2.28× |
| 1,000 | 5 | 0.210 | 0.851 | 4.05× |
| 1,000 | 10 | 0.469 | 1.644 | 3.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
