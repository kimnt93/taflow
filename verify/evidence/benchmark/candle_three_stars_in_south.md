# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.64M | 0.004 | 223.27M | 0.033 | 4.31× | 7.36× |
| 10,000 | 0.067 | 148.78M | 0.062 | 161.86M | 0.110 | 1.64× | 1.78× |
| 100,000 | 0.761 | 131.33M | 0.736 | 135.93M | 0.879 | 1.15× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.113 | 1.28× |
| 1 | 5 | 0.246 | 0.471 | 1.91× |
| 1 | 10 | 0.465 | 0.933 | 2.01× |
| 10 | 1 | 0.045 | 0.089 | 1.95× |
| 10 | 5 | 0.174 | 0.427 | 2.45× |
| 10 | 10 | 0.421 | 0.963 | 2.29× |
| 100 | 1 | 0.040 | 0.085 | 2.10× |
| 100 | 5 | 0.202 | 0.446 | 2.21× |
| 100 | 10 | 0.399 | 1.006 | 2.52× |
| 1,000 | 1 | 0.052 | 0.099 | 1.90× |
| 1,000 | 5 | 0.198 | 0.471 | 2.38× |
| 1,000 | 10 | 0.454 | 1.002 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
