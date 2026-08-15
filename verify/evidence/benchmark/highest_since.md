# HighestSince benchmark (`highest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.89M | 0.004 | 230.76M | 0.274 | 50.46× | 63.31× |
| 10,000 | 0.036 | 274.26M | 0.033 | 306.47M | 2.622 | 71.92× | 80.37× |
| 100,000 | 0.333 | 299.86M | 0.315 | 317.00M | 28.259 | 84.74× | 89.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.095 | 1.17× |
| 1 | 5 | 0.248 | 0.409 | 1.65× |
| 1 | 10 | 0.485 | 0.850 | 1.75× |
| 10 | 1 | 0.045 | 0.082 | 1.83× |
| 10 | 5 | 0.217 | 0.363 | 1.67× |
| 10 | 10 | 0.421 | 0.800 | 1.90× |
| 100 | 1 | 0.046 | 0.094 | 2.04× |
| 100 | 5 | 0.214 | 0.463 | 2.16× |
| 100 | 10 | 0.435 | 1.055 | 2.42× |
| 1,000 | 1 | 0.049 | 0.362 | 7.36× |
| 1,000 | 5 | 0.231 | 2.094 | 9.05× |
| 1,000 | 10 | 0.493 | 3.855 | 7.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
