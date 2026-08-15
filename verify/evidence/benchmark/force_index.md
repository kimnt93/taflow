# ForceIndex benchmark (`ForceIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.02M | 0.005 | 215.71M | 0.227 | 36.82× | 49.02× |
| 10,000 | 0.041 | 244.51M | 0.038 | 264.84M | 0.802 | 19.60× | 21.23× |
| 100,000 | 0.406 | 246.00M | 0.358 | 279.08M | 6.510 | 16.01× | 18.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.263 | 3.44× |
| 1 | 5 | 0.266 | 1.112 | 4.18× |
| 1 | 10 | 0.418 | 2.379 | 5.70× |
| 10 | 1 | 0.044 | 0.221 | 5.03× |
| 10 | 5 | 0.193 | 1.332 | 6.91× |
| 10 | 10 | 0.406 | 2.536 | 6.25× |
| 100 | 1 | 0.056 | 0.221 | 3.92× |
| 100 | 5 | 0.180 | 1.382 | 7.70× |
| 100 | 10 | 0.419 | 2.459 | 5.87× |
| 1,000 | 1 | 0.052 | 0.274 | 5.23× |
| 1,000 | 5 | 0.185 | 1.744 | 9.40× |
| 1,000 | 10 | 0.512 | 3.046 | 5.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
