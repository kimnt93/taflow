# WedgePattern benchmark (`Wedge` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.36M | 0.006 | 160.43M | 0.237 | 24.78× | 38.10× |
| 10,000 | 0.087 | 115.31M | 0.083 | 120.97M | 1.349 | 15.55× | 16.32× |
| 100,000 | 0.810 | 123.46M | 0.797 | 125.46M | 12.859 | 15.88× | 16.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.237 | 2.67× |
| 1 | 5 | 0.291 | 0.808 | 2.78× |
| 1 | 10 | 0.444 | 1.693 | 3.81× |
| 10 | 1 | 0.043 | 0.158 | 3.67× |
| 10 | 5 | 0.183 | 1.080 | 5.90× |
| 10 | 10 | 0.441 | 1.683 | 3.82× |
| 100 | 1 | 0.052 | 0.176 | 3.36× |
| 100 | 5 | 0.202 | 1.190 | 5.88× |
| 100 | 10 | 0.412 | 1.778 | 4.31× |
| 1,000 | 1 | 0.054 | 0.297 | 5.52× |
| 1,000 | 5 | 0.203 | 1.885 | 9.27× |
| 1,000 | 10 | 0.406 | 3.051 | 7.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
