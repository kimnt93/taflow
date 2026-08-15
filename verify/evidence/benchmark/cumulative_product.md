# CumulativeProduct benchmark (`numpy.cumprod` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 249.91M | 0.003 | 365.29M | 0.021 | 5.16× | 7.54× |
| 10,000 | 0.016 | 624.17M | 0.013 | 766.02M | 0.040 | 2.47× | 3.04× |
| 100,000 | 0.123 | 813.32M | 0.095 | 1.05G | 0.222 | 1.81× | 2.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.077 | 1.37× |
| 1 | 5 | 0.294 | 0.321 | 1.09× |
| 1 | 10 | 0.431 | 0.671 | 1.56× |
| 10 | 1 | 0.045 | 0.065 | 1.45× |
| 10 | 5 | 0.178 | 0.312 | 1.76× |
| 10 | 10 | 0.387 | 0.619 | 1.60× |
| 100 | 1 | 0.048 | 0.068 | 1.42× |
| 100 | 5 | 0.203 | 0.326 | 1.61× |
| 100 | 10 | 0.380 | 0.619 | 1.63× |
| 1,000 | 1 | 0.042 | 0.064 | 1.53× |
| 1,000 | 5 | 0.205 | 0.365 | 1.78× |
| 1,000 | 10 | 0.428 | 0.771 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
