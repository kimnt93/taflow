# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.09M | 0.024 | 41.92M | 0.026 | 1.05× | 1.10× |
| 10,000 | 0.148 | 67.70M | 0.139 | 72.06M | 0.039 | 0.27× | 0.28× |
| 100,000 | 1.411 | 70.86M | 1.339 | 74.67M | 0.149 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.132 | 1.32× |
| 1 | 5 | 0.357 | 0.445 | 1.25× |
| 1 | 10 | 0.562 | 0.883 | 1.57× |
| 10 | 1 | 0.060 | 0.088 | 1.47× |
| 10 | 5 | 0.285 | 0.430 | 1.51× |
| 10 | 10 | 0.584 | 0.859 | 1.47× |
| 100 | 1 | 0.061 | 0.089 | 1.44× |
| 100 | 5 | 0.281 | 0.421 | 1.50× |
| 100 | 10 | 0.590 | 0.875 | 1.48× |
| 1,000 | 1 | 0.079 | 0.083 | 1.05× |
| 1,000 | 5 | 0.282 | 0.431 | 1.53× |
| 1,000 | 10 | 0.610 | 0.892 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
