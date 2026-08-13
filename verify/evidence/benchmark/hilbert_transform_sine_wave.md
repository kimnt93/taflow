# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.661 | 1.51M | 0.664 | 1.51M | 0.467 | 0.71× | 0.70× |
| 10,000 | 6.912 | 1.45M | 6.945 | 1.44M | 4.602 | 0.67× | 0.66× |
| 100,000 | 70.801 | 1.41M | 68.844 | 1.45M | 44.598 | 0.63× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.199 | 0.144 | 0.72× |
| 1 | 5 | 0.465 | 0.488 | 1.05× |
| 1 | 10 | 0.614 | 0.902 | 1.47× |
| 10 | 1 | 0.062 | 0.091 | 1.48× |
| 10 | 5 | 0.290 | 0.437 | 1.51× |
| 10 | 10 | 0.605 | 0.933 | 1.54× |
| 100 | 1 | 0.115 | 0.121 | 1.05× |
| 100 | 5 | 0.300 | 0.587 | 1.96× |
| 100 | 10 | 0.662 | 1.206 | 1.82× |
| 1,000 | 1 | 0.778 | 0.558 | 0.72× |
| 1,000 | 5 | 0.945 | 2.773 | 2.93× |
| 1,000 | 10 | 1.619 | 5.586 | 3.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
