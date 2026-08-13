# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.95M | 0.076 | 13.14M | 0.032 | 0.39× | 0.43× |
| 10,000 | 0.604 | 16.55M | 0.600 | 16.66M | 0.106 | 0.18× | 0.18× |
| 100,000 | 5.874 | 17.03M | 5.770 | 17.33M | 0.843 | 0.14× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.101 | 0.62× |
| 1 | 5 | 0.455 | 0.490 | 1.08× |
| 1 | 10 | 0.631 | 0.900 | 1.43× |
| 10 | 1 | 0.071 | 0.088 | 1.24× |
| 10 | 5 | 0.315 | 0.424 | 1.35× |
| 10 | 10 | 0.627 | 0.903 | 1.44× |
| 100 | 1 | 0.088 | 0.091 | 1.04× |
| 100 | 5 | 0.308 | 0.420 | 1.36× |
| 100 | 10 | 0.649 | 0.896 | 1.38× |
| 1,000 | 1 | 0.140 | 0.098 | 0.70× |
| 1,000 | 5 | 0.328 | 0.488 | 1.49× |
| 1,000 | 10 | 0.665 | 0.979 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
