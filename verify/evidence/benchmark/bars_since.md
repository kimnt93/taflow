# BarsSince benchmark (`bars since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 232.10M | 0.003 | 316.22M | 0.113 | 26.14× | 35.61× |
| 10,000 | 0.026 | 386.65M | 0.024 | 420.72M | 1.057 | 40.87× | 44.48× |
| 100,000 | 0.240 | 416.82M | 0.234 | 427.98M | 10.785 | 44.95× | 46.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.084 | 0.65× |
| 1 | 5 | 0.271 | 0.295 | 1.09× |
| 1 | 10 | 0.363 | 0.611 | 1.68× |
| 10 | 1 | 0.049 | 0.072 | 1.47× |
| 10 | 5 | 0.177 | 0.312 | 1.76× |
| 10 | 10 | 0.386 | 0.631 | 1.64× |
| 100 | 1 | 0.041 | 0.078 | 1.91× |
| 100 | 5 | 0.183 | 0.342 | 1.87× |
| 100 | 10 | 0.412 | 0.712 | 1.73× |
| 1,000 | 1 | 0.049 | 0.163 | 3.31× |
| 1,000 | 5 | 0.196 | 0.804 | 4.10× |
| 1,000 | 10 | 0.437 | 1.810 | 4.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
