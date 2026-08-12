# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.939 | 515.78K | 1.953 | 512.04K | 8.446 | 4.36× | 4.32× |
| 10,000 | 20.296 | 492.72K | 20.387 | 490.50K | 86.817 | 4.28× | 4.26× |
| 100,000 | 205.792 | 485.93K | 221.678 | 451.10K | 878.023 | 4.27× | 3.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.454 | 3.68× |
| 1 | 5 | 0.310 | 1.387 | 4.48× |
| 1 | 10 | 0.525 | 3.118 | 5.93× |
| 10 | 1 | 0.065 | 0.293 | 4.50× |
| 10 | 5 | 0.272 | 1.480 | 5.44× |
| 10 | 10 | 0.514 | 2.905 | 5.65× |
| 100 | 1 | 0.142 | 0.650 | 4.57× |
| 100 | 5 | 0.344 | 3.855 | 11.22× |
| 100 | 10 | 0.611 | 7.491 | 12.26× |
| 1,000 | 1 | 1.986 | 8.083 | 4.07× |
| 1,000 | 5 | 3.816 | 52.456 | 13.75× |
| 1,000 | 10 | 4.506 | 94.681 | 21.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
