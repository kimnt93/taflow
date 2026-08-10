# CumulativeMinimum benchmark (`numpy.minimum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.40M | 0.005 | 217.68M | 0.016 | 2.94× | 3.47× |
| 10,000 | 0.031 | 325.86M | 0.029 | 350.68M | 0.038 | 1.24× | 1.34× |
| 100,000 | 0.284 | 352.53M | 0.256 | 391.13M | 0.270 | 0.95× | 1.05× |
| 1,000,000 | 3.134 | 319.08M | 2.777 | 360.11M | 2.793 | 0.89× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.067 | 0.79× |
| 1 | 5 | 0.280 | 0.321 | 1.15× |
| 1 | 10 | 0.473 | 0.623 | 1.32× |
| 10 | 1 | 0.046 | 0.056 | 1.22× |
| 10 | 5 | 0.228 | 0.290 | 1.28× |
| 10 | 10 | 0.455 | 0.626 | 1.38× |
| 100 | 1 | 0.045 | 0.062 | 1.37× |
| 100 | 5 | 0.231 | 0.297 | 1.28× |
| 100 | 10 | 0.482 | 0.616 | 1.28× |
| 1,000 | 1 | 0.050 | 0.062 | 1.25× |
| 1,000 | 5 | 0.235 | 0.326 | 1.39× |
| 1,000 | 10 | 0.526 | 0.769 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
