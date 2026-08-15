# RollingSortino benchmark (`SortinoRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.04M | 0.021 | 48.36M | 0.215 | 9.04× | 10.40× |
| 10,000 | 0.185 | 53.97M | 0.191 | 52.36M | 1.275 | 6.88× | 6.68× |
| 100,000 | 3.223 | 31.02M | 3.282 | 30.47M | 7.733 | 2.40× | 2.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.252 | 3.01× |
| 1 | 5 | 0.282 | 1.273 | 4.51× |
| 1 | 10 | 0.448 | 2.606 | 5.82× |
| 10 | 1 | 0.046 | 0.237 | 5.19× |
| 10 | 5 | 0.218 | 1.191 | 5.46× |
| 10 | 10 | 0.464 | 2.659 | 5.73× |
| 100 | 1 | 0.052 | 0.225 | 4.36× |
| 100 | 5 | 0.234 | 1.361 | 5.81× |
| 100 | 10 | 0.485 | 2.702 | 5.57× |
| 1,000 | 1 | 0.066 | 0.272 | 4.12× |
| 1,000 | 5 | 0.229 | 1.792 | 7.82× |
| 1,000 | 10 | 0.479 | 3.368 | 7.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
