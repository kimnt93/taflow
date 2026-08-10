# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.42M | 0.004 | 239.69M | 0.033 | 6.00× | 7.97× |
| 10,000 | 0.012 | 830.78M | 0.008 | 1.22G | 0.039 | 3.21× | 4.70× |
| 100,000 | 0.080 | 1.25G | 0.049 | 2.05G | 0.082 | 1.03× | 1.69× |
| 1,000,000 | 1.783 | 560.93M | 1.563 | 639.81M | 1.437 | 0.81× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.104 | 1.19× |
| 1 | 5 | 0.312 | 0.498 | 1.60× |
| 1 | 10 | 0.575 | 1.157 | 2.01× |
| 10 | 1 | 0.057 | 0.091 | 1.60× |
| 10 | 5 | 0.262 | 0.585 | 2.24× |
| 10 | 10 | 0.631 | 1.199 | 1.90× |
| 100 | 1 | 0.062 | 0.107 | 1.73× |
| 100 | 5 | 0.273 | 0.592 | 2.17× |
| 100 | 10 | 0.617 | 1.166 | 1.89× |
| 1,000 | 1 | 0.062 | 0.099 | 1.60× |
| 1,000 | 5 | 0.295 | 0.520 | 1.76× |
| 1,000 | 10 | 0.531 | 1.012 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
