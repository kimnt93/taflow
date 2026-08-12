# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.14M | 0.027 | 36.62M | 0.245 | 8.59× | 8.96× |
| 10,000 | 0.229 | 43.68M | 0.234 | 42.71M | 1.011 | 4.42× | 4.32× |
| 100,000 | 2.393 | 41.79M | 2.345 | 42.65M | 9.331 | 3.90× | 3.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.339 | 4.79× |
| 1 | 5 | 0.380 | 1.350 | 3.55× |
| 1 | 10 | 0.594 | 2.663 | 4.48× |
| 10 | 1 | 0.051 | 0.242 | 4.76× |
| 10 | 5 | 0.285 | 1.438 | 5.05× |
| 10 | 10 | 0.529 | 2.730 | 5.16× |
| 100 | 1 | 0.065 | 0.306 | 4.70× |
| 100 | 5 | 0.261 | 1.404 | 5.38× |
| 100 | 10 | 0.583 | 2.832 | 4.85× |
| 1,000 | 1 | 0.094 | 0.325 | 3.47× |
| 1,000 | 5 | 0.324 | 1.933 | 5.97× |
| 1,000 | 10 | 0.616 | 3.890 | 6.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
