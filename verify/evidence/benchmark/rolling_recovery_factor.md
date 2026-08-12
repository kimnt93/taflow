# RollingRecoveryFactor benchmark (`rolling recovery factor on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.55M | 0.063 | 15.99M | 0.222 | 3.45× | 3.55× |
| 10,000 | 0.599 | 16.70M | 0.599 | 16.69M | 1.424 | 2.38× | 2.38× |
| 100,000 | 5.879 | 17.01M | 5.912 | 16.91M | 19.804 | 3.37× | 3.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.119 | 1.44× |
| 1 | 5 | 0.283 | 0.542 | 1.92× |
| 1 | 10 | 0.525 | 1.157 | 2.20× |
| 10 | 1 | 0.056 | 0.091 | 1.63× |
| 10 | 5 | 0.248 | 0.473 | 1.90× |
| 10 | 10 | 0.609 | 0.977 | 1.60× |
| 100 | 1 | 0.062 | 0.193 | 3.10× |
| 100 | 5 | 0.263 | 1.134 | 4.32× |
| 100 | 10 | 0.636 | 2.228 | 3.50× |
| 1,000 | 1 | 0.116 | 0.327 | 2.81× |
| 1,000 | 5 | 0.334 | 1.452 | 4.34× |
| 1,000 | 10 | 0.642 | 3.332 | 5.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
