# CumulativeMaximum benchmark (`numpy.maximum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 233.74M | 0.003 | 297.56M | 0.016 | 3.83× | 4.88× |
| 10,000 | 0.031 | 318.82M | 0.027 | 365.98M | 0.040 | 1.29× | 1.48× |
| 100,000 | 0.280 | 356.70M | 0.260 | 383.96M | 0.285 | 1.02× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.067 | 1.03× |
| 1 | 5 | 0.323 | 0.302 | 0.94× |
| 1 | 10 | 0.374 | 0.595 | 1.59× |
| 10 | 1 | 0.041 | 0.057 | 1.39× |
| 10 | 5 | 0.172 | 0.325 | 1.88× |
| 10 | 10 | 0.379 | 0.590 | 1.56× |
| 100 | 1 | 0.042 | 0.057 | 1.34× |
| 100 | 5 | 0.177 | 0.279 | 1.58× |
| 100 | 10 | 0.378 | 0.610 | 1.61× |
| 1,000 | 1 | 0.046 | 0.063 | 1.37× |
| 1,000 | 5 | 0.202 | 0.312 | 1.55× |
| 1,000 | 10 | 0.454 | 0.733 | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
