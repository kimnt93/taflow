# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.41M | 0.010 | 99.76M | 0.050 | 4.49× | 4.95× |
| 10,000 | 0.061 | 163.12M | 0.060 | 167.54M | 0.091 | 1.48× | 1.53× |
| 100,000 | 0.601 | 166.43M | 0.602 | 165.99M | 0.698 | 1.16× | 1.16× |
| 1,000,000 | 6.315 | 158.36M | 5.868 | 170.41M | 6.708 | 1.06× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.191 | 1.45× |
| 1 | 5 | 0.402 | 0.965 | 2.40× |
| 1 | 10 | 0.890 | 4.393 | 4.93× |
| 10 | 1 | 0.385 | 0.354 | 0.92× |
| 10 | 5 | 0.338 | 0.589 | 1.74× |
| 10 | 10 | 0.713 | 1.218 | 1.71× |
| 100 | 1 | 0.094 | 0.132 | 1.40× |
| 100 | 5 | 0.448 | 0.922 | 2.06× |
| 100 | 10 | 0.738 | 1.226 | 1.66× |
| 1,000 | 1 | 0.062 | 0.113 | 1.83× |
| 1,000 | 5 | 0.393 | 0.691 | 1.76× |
| 1,000 | 10 | 0.677 | 1.110 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
