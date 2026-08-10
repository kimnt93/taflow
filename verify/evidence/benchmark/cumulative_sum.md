# CumulativeSum benchmark (`numpy.cumsum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 216.34M | 0.003 | 293.59M | 0.017 | 3.71× | 5.03× |
| 10,000 | 0.016 | 633.70M | 0.013 | 750.68M | 0.038 | 2.42× | 2.87× |
| 100,000 | 0.151 | 662.57M | 0.114 | 878.76M | 0.282 | 1.87× | 2.48× |
| 1,000,000 | 1.981 | 504.89M | 1.355 | 738.23M | 2.356 | 1.19× | 1.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.088 | 0.77× |
| 1 | 5 | 0.273 | 0.340 | 1.25× |
| 1 | 10 | 0.486 | 0.645 | 1.33× |
| 10 | 1 | 0.051 | 0.075 | 1.47× |
| 10 | 5 | 0.275 | 0.350 | 1.27× |
| 10 | 10 | 0.498 | 0.675 | 1.36× |
| 100 | 1 | 0.050 | 0.066 | 1.32× |
| 100 | 5 | 0.227 | 0.337 | 1.48× |
| 100 | 10 | 0.522 | 0.787 | 1.51× |
| 1,000 | 1 | 0.057 | 0.066 | 1.15× |
| 1,000 | 5 | 0.248 | 0.374 | 1.51× |
| 1,000 | 10 | 0.472 | 0.887 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
