# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.96M | 0.004 | 265.07M | 0.032 | 6.13× | 8.46× |
| 10,000 | 0.011 | 871.20M | 0.008 | 1.22G | 0.038 | 3.27× | 4.60× |
| 100,000 | 0.079 | 1.27G | 0.050 | 2.01G | 0.079 | 1.00× | 1.59× |
| 1,000,000 | 1.818 | 550.06M | 1.362 | 734.17M | 1.425 | 0.78× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.107 | 1.33× |
| 1 | 5 | 0.373 | 0.594 | 1.60× |
| 1 | 10 | 0.539 | 1.120 | 2.08× |
| 10 | 1 | 0.055 | 0.103 | 1.88× |
| 10 | 5 | 0.360 | 0.510 | 1.42× |
| 10 | 10 | 0.697 | 1.134 | 1.63× |
| 100 | 1 | 0.063 | 0.105 | 1.67× |
| 100 | 5 | 0.281 | 0.552 | 1.97× |
| 100 | 10 | 0.650 | 1.197 | 1.84× |
| 1,000 | 1 | 0.060 | 0.107 | 1.80× |
| 1,000 | 5 | 0.299 | 0.820 | 2.75× |
| 1,000 | 10 | 0.552 | 1.049 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
