# HeikinAshi benchmark (`HeikinAshi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.60M | 0.008 | 124.48M | 0.578 | 54.10× | 71.95× |
| 10,000 | 0.081 | 123.94M | 0.069 | 145.63M | 4.798 | 59.46× | 69.87× |
| 100,000 | 0.778 | 128.55M | 0.677 | 147.78M | 54.577 | 70.16× | 80.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.235 | 2.37× |
| 1 | 5 | 0.265 | 0.947 | 3.57× |
| 1 | 10 | 0.411 | 2.077 | 5.06× |
| 10 | 1 | 0.044 | 0.194 | 4.40× |
| 10 | 5 | 0.181 | 0.930 | 5.14× |
| 10 | 10 | 0.396 | 2.058 | 5.20× |
| 100 | 1 | 0.049 | 0.235 | 4.77× |
| 100 | 5 | 0.216 | 1.174 | 5.43× |
| 100 | 10 | 0.408 | 2.596 | 6.36× |
| 1,000 | 1 | 0.060 | 0.992 | 16.47× |
| 1,000 | 5 | 0.208 | 3.713 | 17.86× |
| 1,000 | 10 | 0.491 | 7.670 | 15.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
