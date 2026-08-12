# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.85M | 0.005 | 208.35M | 0.034 | 5.92× | 7.06× |
| 10,000 | 0.034 | 292.21M | 0.033 | 305.08M | 0.076 | 2.22× | 2.32× |
| 100,000 | 0.342 | 292.41M | 0.313 | 319.49M | 0.489 | 1.43× | 1.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.111 | 1.30× |
| 1 | 5 | 0.382 | 0.520 | 1.36× |
| 1 | 10 | 0.476 | 0.927 | 1.95× |
| 10 | 1 | 0.050 | 0.091 | 1.83× |
| 10 | 5 | 0.222 | 0.432 | 1.95× |
| 10 | 10 | 0.469 | 0.908 | 1.94× |
| 100 | 1 | 0.048 | 0.098 | 2.06× |
| 100 | 5 | 0.225 | 0.445 | 1.97× |
| 100 | 10 | 0.501 | 0.927 | 1.85× |
| 1,000 | 1 | 0.059 | 0.096 | 1.64× |
| 1,000 | 5 | 0.238 | 0.473 | 1.99× |
| 1,000 | 10 | 0.472 | 0.989 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
