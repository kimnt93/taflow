# PreviousHighLow benchmark (`previous-session high-low` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.51M | 0.076 | 13.22M | 0.565 | 6.51× | 7.47× |
| 10,000 | 0.660 | 15.14M | 0.656 | 15.25M | 5.769 | 8.74× | 8.80× |
| 100,000 | 6.426 | 15.56M | 6.257 | 15.98M | 57.356 | 8.93× | 9.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.184 | 0.211 | 1.15× |
| 1 | 5 | 0.353 | 0.338 | 0.96× |
| 1 | 10 | 0.634 | 0.680 | 1.07× |
| 10 | 1 | 0.068 | 0.073 | 1.07× |
| 10 | 5 | 0.312 | 0.344 | 1.10× |
| 10 | 10 | 0.625 | 0.714 | 1.14× |
| 100 | 1 | 0.074 | 0.126 | 1.71× |
| 100 | 5 | 0.308 | 0.598 | 1.94× |
| 100 | 10 | 0.650 | 1.231 | 1.89× |
| 1,000 | 1 | 0.141 | 0.663 | 4.70× |
| 1,000 | 5 | 0.315 | 3.318 | 10.54× |
| 1,000 | 10 | 0.675 | 6.758 | 10.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
