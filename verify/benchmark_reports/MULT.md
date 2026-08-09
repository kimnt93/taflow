# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.08M | 0.003 | 293.05M | 0.031 | 6.16× | 9.06× |
| 10,000 | 0.011 | 917.91M | 0.008 | 1.22G | 0.035 | 3.20× | 4.26× |
| 100,000 | 0.070 | 1.44G | 0.042 | 2.41G | 0.071 | 1.02× | 1.70× |
| 1,000,000 | 2.212 | 452.14M | 0.921 | 1.09G | 1.218 | 0.55× | 1.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.176 | 1.31× |
| 1 | 5 | 0.301 | 0.516 | 1.71× |
| 1 | 10 | 0.568 | 1.071 | 1.89× |
| 10 | 1 | 0.053 | 0.098 | 1.82× |
| 10 | 5 | 0.254 | 0.469 | 1.85× |
| 10 | 10 | 0.592 | 1.041 | 1.76× |
| 100 | 1 | 0.050 | 0.091 | 1.81× |
| 100 | 5 | 0.257 | 0.468 | 1.83× |
| 100 | 10 | 0.563 | 1.043 | 1.85× |
| 1,000 | 1 | 0.052 | 0.093 | 1.77× |
| 1,000 | 5 | 0.250 | 0.507 | 2.03× |
| 1,000 | 10 | 0.546 | 1.048 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
