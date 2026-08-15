# LowestSince benchmark (`lowest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.82M | 0.005 | 211.73M | 0.272 | 44.80× | 57.55× |
| 10,000 | 0.037 | 267.06M | 0.038 | 264.80M | 2.855 | 76.25× | 75.61× |
| 100,000 | 0.336 | 297.96M | 0.309 | 323.67M | 27.547 | 82.08× | 89.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.102 | 1.38× |
| 1 | 5 | 0.296 | 0.338 | 1.14× |
| 1 | 10 | 0.400 | 0.674 | 1.68× |
| 10 | 1 | 0.041 | 0.074 | 1.82× |
| 10 | 5 | 0.174 | 0.320 | 1.85× |
| 10 | 10 | 0.393 | 0.715 | 1.82× |
| 100 | 1 | 0.042 | 0.091 | 2.18× |
| 100 | 5 | 0.182 | 0.459 | 2.52× |
| 100 | 10 | 0.382 | 0.930 | 2.44× |
| 1,000 | 1 | 0.046 | 0.355 | 7.64× |
| 1,000 | 5 | 0.199 | 1.709 | 8.60× |
| 1,000 | 10 | 0.422 | 3.445 | 8.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
