# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.39M | 0.024 | 42.04M | 0.212 | 8.55× | 8.90× |
| 10,000 | 0.205 | 48.86M | 0.208 | 48.11M | 0.892 | 4.36× | 4.29× |
| 100,000 | 2.033 | 49.20M | 1.982 | 50.45M | 7.800 | 3.84× | 3.93× |
| 1,000,000 | 20.791 | 48.10M | 20.835 | 48.00M | 74.571 | 3.59× | 3.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.341 | 2.29× |
| 1 | 5 | 0.333 | 1.292 | 3.88× |
| 1 | 10 | 0.493 | 2.336 | 4.74× |
| 10 | 1 | 0.050 | 0.226 | 4.49× |
| 10 | 5 | 0.227 | 1.257 | 5.54× |
| 10 | 10 | 0.508 | 2.326 | 4.58× |
| 100 | 1 | 0.056 | 0.244 | 4.36× |
| 100 | 5 | 0.253 | 1.321 | 5.22× |
| 100 | 10 | 0.525 | 2.453 | 4.67× |
| 1,000 | 1 | 0.072 | 0.302 | 4.22× |
| 1,000 | 5 | 0.242 | 1.680 | 6.94× |
| 1,000 | 10 | 0.514 | 3.210 | 6.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
