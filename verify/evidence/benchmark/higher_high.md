# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.73M | 0.007 | 151.72M | 0.017 | 2.27× | 2.54× |
| 10,000 | 0.032 | 309.93M | 0.028 | 352.49M | 0.024 | 0.74× | 0.84× |
| 100,000 | 0.268 | 372.80M | 0.245 | 407.65M | 0.115 | 0.43× | 0.47× |
| 1,000,000 | 3.185 | 313.98M | 2.579 | 387.69M | 1.669 | 0.52× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.080 | 0.97× |
| 1 | 5 | 0.381 | 0.323 | 0.85× |
| 1 | 10 | 0.460 | 0.683 | 1.49× |
| 10 | 1 | 0.051 | 0.066 | 1.32× |
| 10 | 5 | 0.224 | 0.334 | 1.49× |
| 10 | 10 | 0.469 | 0.675 | 1.44× |
| 100 | 1 | 0.053 | 0.069 | 1.30× |
| 100 | 5 | 0.240 | 0.317 | 1.32× |
| 100 | 10 | 0.501 | 0.682 | 1.36× |
| 1,000 | 1 | 0.050 | 0.066 | 1.33× |
| 1,000 | 5 | 0.235 | 0.348 | 1.48× |
| 1,000 | 10 | 0.487 | 0.791 | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
