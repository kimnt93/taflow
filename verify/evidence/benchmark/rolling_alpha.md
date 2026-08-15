# RollingAlpha benchmark (`Alpha` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.69M | 0.039 | 25.43M | 0.232 | 5.73× | 5.90× |
| 10,000 | 0.390 | 25.65M | 0.378 | 26.46M | 0.927 | 2.38× | 2.45× |
| 100,000 | 4.156 | 24.06M | 4.080 | 24.51M | 8.092 | 1.95× | 1.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.273 | 2.58× |
| 1 | 5 | 0.302 | 1.132 | 3.75× |
| 1 | 10 | 0.417 | 2.511 | 6.03× |
| 10 | 1 | 0.047 | 0.228 | 4.82× |
| 10 | 5 | 0.184 | 1.420 | 7.71× |
| 10 | 10 | 0.412 | 2.528 | 6.14× |
| 100 | 1 | 0.047 | 0.231 | 4.90× |
| 100 | 5 | 0.202 | 1.473 | 7.30× |
| 100 | 10 | 0.474 | 2.579 | 5.44× |
| 1,000 | 1 | 0.090 | 0.312 | 3.48× |
| 1,000 | 5 | 0.243 | 1.895 | 7.80× |
| 1,000 | 10 | 0.460 | 3.381 | 7.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
