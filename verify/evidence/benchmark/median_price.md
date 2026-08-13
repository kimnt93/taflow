# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.43M | 0.021 | 47.72M | 0.028 | 1.05× | 1.34× |
| 10,000 | 0.152 | 65.97M | 0.141 | 70.67M | 0.033 | 0.22× | 0.23× |
| 100,000 | 1.309 | 76.40M | 1.313 | 76.19M | 0.069 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.105 | 1.11× |
| 1 | 5 | 0.426 | 0.468 | 1.10× |
| 1 | 10 | 0.671 | 0.902 | 1.35× |
| 10 | 1 | 0.068 | 0.090 | 1.33× |
| 10 | 5 | 0.292 | 0.425 | 1.46× |
| 10 | 10 | 0.585 | 0.894 | 1.53× |
| 100 | 1 | 0.061 | 0.086 | 1.42× |
| 100 | 5 | 0.303 | 0.410 | 1.36× |
| 100 | 10 | 0.592 | 0.906 | 1.53× |
| 1,000 | 1 | 0.076 | 0.090 | 1.17× |
| 1,000 | 5 | 0.291 | 0.406 | 1.40× |
| 1,000 | 10 | 0.608 | 0.887 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
