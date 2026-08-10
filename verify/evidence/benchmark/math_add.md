# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 204.05M | 0.003 | 289.74M | 0.033 | 6.66× | 9.46× |
| 10,000 | 0.010 | 984.47M | 0.007 | 1.47G | 0.034 | 3.32× | 4.96× |
| 100,000 | 0.067 | 1.48G | 0.044 | 2.29G | 0.074 | 1.09× | 1.69× |
| 1,000,000 | 1.291 | 774.49M | 0.950 | 1.05G | 1.035 | 0.80× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.097 | 1.16× |
| 1 | 5 | 0.339 | 0.432 | 1.28× |
| 1 | 10 | 0.470 | 0.878 | 1.87× |
| 10 | 1 | 0.050 | 0.086 | 1.74× |
| 10 | 5 | 0.246 | 0.472 | 1.92× |
| 10 | 10 | 0.467 | 0.895 | 1.92× |
| 100 | 1 | 0.048 | 0.093 | 1.96× |
| 100 | 5 | 0.229 | 0.459 | 2.01× |
| 100 | 10 | 0.506 | 0.871 | 1.72× |
| 1,000 | 1 | 0.048 | 0.086 | 1.78× |
| 1,000 | 5 | 0.223 | 0.419 | 1.88× |
| 1,000 | 10 | 0.451 | 0.978 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
