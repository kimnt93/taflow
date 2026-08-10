# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.34M | 0.012 | 84.81M | 0.039 | 2.52× | 3.32× |
| 10,000 | 0.104 | 96.51M | 0.081 | 123.92M | 0.141 | 1.36× | 1.75× |
| 100,000 | 0.971 | 102.97M | 0.945 | 105.83M | 0.990 | 1.02× | 1.05× |
| 1,000,000 | 9.532 | 104.91M | 10.085 | 99.16M | 11.257 | 1.18× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.130 | 1.66× |
| 1 | 5 | 0.351 | 0.574 | 1.64× |
| 1 | 10 | 0.710 | 1.179 | 1.66× |
| 10 | 1 | 0.077 | 0.121 | 1.58× |
| 10 | 5 | 0.337 | 0.583 | 1.73× |
| 10 | 10 | 0.719 | 1.196 | 1.66× |
| 100 | 1 | 0.083 | 0.113 | 1.37× |
| 100 | 5 | 0.327 | 0.546 | 1.67× |
| 100 | 10 | 0.680 | 1.089 | 1.60× |
| 1,000 | 1 | 0.088 | 0.116 | 1.32× |
| 1,000 | 5 | 0.332 | 0.594 | 1.79× |
| 1,000 | 10 | 0.630 | 1.119 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
