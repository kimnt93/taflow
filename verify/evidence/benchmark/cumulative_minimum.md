# CumulativeMinimum benchmark (`numpy.minimum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.13M | 0.005 | 193.54M | 0.016 | 2.42× | 3.10× |
| 10,000 | 0.036 | 276.05M | 0.033 | 305.62M | 0.042 | 1.16× | 1.28× |
| 100,000 | 0.306 | 327.04M | 0.276 | 362.12M | 0.282 | 0.92× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.060 | 0.56× |
| 1 | 5 | 0.268 | 0.288 | 1.08× |
| 1 | 10 | 0.492 | 0.609 | 1.24× |
| 10 | 1 | 0.047 | 0.059 | 1.25× |
| 10 | 5 | 0.229 | 0.282 | 1.23× |
| 10 | 10 | 0.490 | 0.624 | 1.27× |
| 100 | 1 | 0.057 | 0.074 | 1.30× |
| 100 | 5 | 0.247 | 0.344 | 1.39× |
| 100 | 10 | 0.517 | 0.599 | 1.16× |
| 1,000 | 1 | 0.052 | 0.060 | 1.17× |
| 1,000 | 5 | 0.217 | 0.321 | 1.48× |
| 1,000 | 10 | 0.510 | 0.781 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
