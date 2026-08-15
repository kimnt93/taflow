# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.03M | 0.005 | 195.74M | 0.034 | 4.45× | 6.69× |
| 10,000 | 0.073 | 137.78M | 0.062 | 160.23M | 0.126 | 1.73× | 2.01× |
| 100,000 | 0.771 | 129.72M | 0.754 | 132.59M | 0.911 | 1.18× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.141 | 1.71× |
| 1 | 5 | 0.273 | 0.483 | 1.77× |
| 1 | 10 | 0.385 | 0.908 | 2.36× |
| 10 | 1 | 0.045 | 0.094 | 2.07× |
| 10 | 5 | 0.204 | 0.447 | 2.19× |
| 10 | 10 | 0.395 | 0.889 | 2.25× |
| 100 | 1 | 0.043 | 0.086 | 2.02× |
| 100 | 5 | 0.194 | 0.437 | 2.25× |
| 100 | 10 | 0.451 | 0.942 | 2.09× |
| 1,000 | 1 | 0.047 | 0.103 | 2.20× |
| 1,000 | 5 | 0.202 | 0.494 | 2.44× |
| 1,000 | 10 | 0.379 | 1.032 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
