# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.34M | 0.005 | 202.80M | 0.047 | 5.19× | 9.62× |
| 10,000 | 0.076 | 131.55M | 0.070 | 142.76M | 0.189 | 2.49× | 2.70× |
| 100,000 | 0.835 | 119.74M | 0.818 | 122.29M | 1.632 | 1.95× | 2.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.106 | 0.78× |
| 1 | 5 | 0.208 | 0.460 | 2.21× |
| 1 | 10 | 0.422 | 1.005 | 2.38× |
| 10 | 1 | 0.050 | 0.102 | 2.02× |
| 10 | 5 | 0.188 | 0.438 | 2.33× |
| 10 | 10 | 0.393 | 0.920 | 2.34× |
| 100 | 1 | 0.041 | 0.090 | 2.21× |
| 100 | 5 | 0.190 | 0.481 | 2.53× |
| 100 | 10 | 0.408 | 0.926 | 2.27× |
| 1,000 | 1 | 0.048 | 0.109 | 2.27× |
| 1,000 | 5 | 0.184 | 0.504 | 2.74× |
| 1,000 | 10 | 0.440 | 1.148 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
