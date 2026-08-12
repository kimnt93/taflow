# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.86M | 0.018 | 55.20M | 0.026 | 1.32× | 1.41× |
| 10,000 | 0.164 | 61.10M | 0.160 | 62.35M | 0.148 | 0.90× | 0.92× |
| 100,000 | 1.627 | 61.46M | 1.666 | 60.03M | 1.407 | 0.86× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.074 | 0.60× |
| 1 | 5 | 0.341 | 0.303 | 0.89× |
| 1 | 10 | 0.461 | 0.555 | 1.20× |
| 10 | 1 | 0.050 | 0.060 | 1.20× |
| 10 | 5 | 0.226 | 0.273 | 1.21× |
| 10 | 10 | 0.497 | 0.596 | 1.20× |
| 100 | 1 | 0.050 | 0.056 | 1.13× |
| 100 | 5 | 0.229 | 0.286 | 1.25× |
| 100 | 10 | 0.484 | 0.595 | 1.23× |
| 1,000 | 1 | 0.065 | 0.071 | 1.08× |
| 1,000 | 5 | 0.233 | 0.300 | 1.29× |
| 1,000 | 10 | 0.491 | 0.726 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
