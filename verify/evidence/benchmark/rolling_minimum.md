# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 228.95M | 0.004 | 271.45M | 0.040 | 9.20× | 10.91× |
| 10,000 | 0.024 | 408.44M | 0.022 | 460.98M | 0.079 | 3.23× | 3.64× |
| 100,000 | 0.225 | 444.04M | 0.199 | 502.07M | 0.511 | 2.27× | 2.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.136 | 1.20× |
| 1 | 5 | 0.327 | 0.517 | 1.58× |
| 1 | 10 | 0.399 | 0.926 | 2.32× |
| 10 | 1 | 0.040 | 0.094 | 2.33× |
| 10 | 5 | 0.172 | 0.423 | 2.46× |
| 10 | 10 | 0.363 | 1.013 | 2.79× |
| 100 | 1 | 0.044 | 0.092 | 2.08× |
| 100 | 5 | 0.187 | 0.474 | 2.53× |
| 100 | 10 | 0.416 | 1.024 | 2.46× |
| 1,000 | 1 | 0.055 | 0.110 | 1.99× |
| 1,000 | 5 | 0.218 | 0.483 | 2.22× |
| 1,000 | 10 | 0.407 | 0.948 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
