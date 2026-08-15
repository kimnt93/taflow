# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.63M | 0.019 | 53.76M | 0.027 | 1.38× | 1.46× |
| 10,000 | 0.175 | 57.18M | 0.174 | 57.48M | 0.157 | 0.90× | 0.90× |
| 100,000 | 1.787 | 55.97M | 1.745 | 57.32M | 1.441 | 0.81× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.082 | 1.04× |
| 1 | 5 | 0.366 | 0.312 | 0.85× |
| 1 | 10 | 0.374 | 0.596 | 1.59× |
| 10 | 1 | 0.040 | 0.056 | 1.41× |
| 10 | 5 | 0.176 | 0.291 | 1.65× |
| 10 | 10 | 0.451 | 0.600 | 1.33× |
| 100 | 1 | 0.042 | 0.060 | 1.44× |
| 100 | 5 | 0.200 | 0.297 | 1.49× |
| 100 | 10 | 0.424 | 0.614 | 1.45× |
| 1,000 | 1 | 0.062 | 0.083 | 1.33× |
| 1,000 | 5 | 0.199 | 0.324 | 1.63× |
| 1,000 | 10 | 0.446 | 0.744 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
