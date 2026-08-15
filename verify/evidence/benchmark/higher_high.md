# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 412.42M | 0.001 | 917.37M | 0.017 | 6.92× | 15.39× |
| 10,000 | 0.007 | 1.35G | 0.004 | 2.31G | 0.024 | 3.30× | 5.65× |
| 100,000 | 0.063 | 1.58G | 0.037 | 2.69G | 0.110 | 1.74× | 2.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.085 | 1.20× |
| 1 | 5 | 0.206 | 0.378 | 1.84× |
| 1 | 10 | 0.461 | 0.743 | 1.61× |
| 10 | 1 | 0.043 | 0.070 | 1.62× |
| 10 | 5 | 0.173 | 0.336 | 1.94× |
| 10 | 10 | 0.393 | 0.694 | 1.77× |
| 100 | 1 | 0.045 | 0.078 | 1.72× |
| 100 | 5 | 0.192 | 0.328 | 1.71× |
| 100 | 10 | 0.386 | 0.680 | 1.76× |
| 1,000 | 1 | 0.042 | 0.069 | 1.66× |
| 1,000 | 5 | 0.172 | 0.350 | 2.03× |
| 1,000 | 10 | 0.423 | 0.821 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
