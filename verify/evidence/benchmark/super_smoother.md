# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.58M | 0.007 | 145.78M | 0.148 | 18.09× | 21.51× |
| 10,000 | 0.053 | 190.44M | 0.052 | 190.90M | 0.454 | 8.65× | 8.67× |
| 100,000 | 0.466 | 214.65M | 0.449 | 222.49M | 3.478 | 7.47× | 7.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.239 | 1.74× |
| 1 | 5 | 0.294 | 0.983 | 3.34× |
| 1 | 10 | 0.471 | 2.076 | 4.41× |
| 10 | 1 | 0.056 | 0.213 | 3.80× |
| 10 | 5 | 0.232 | 0.953 | 4.11× |
| 10 | 10 | 0.455 | 2.233 | 4.91× |
| 100 | 1 | 0.059 | 0.194 | 3.29× |
| 100 | 5 | 0.240 | 0.981 | 4.08× |
| 100 | 10 | 0.469 | 2.202 | 4.70× |
| 1,000 | 1 | 0.055 | 0.230 | 4.16× |
| 1,000 | 5 | 0.237 | 1.124 | 4.75× |
| 1,000 | 10 | 0.537 | 2.525 | 4.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
