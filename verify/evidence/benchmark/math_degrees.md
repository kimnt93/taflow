# MathDegrees benchmark (`numpy.degrees` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 318.20M | 0.002 | 420.68M | 0.013 | 4.22× | 5.58× |
| 10,000 | 0.007 | 1.36G | 0.005 | 2.12G | 0.025 | 3.39× | 5.27× |
| 100,000 | 0.058 | 1.74G | 0.031 | 3.18G | 0.134 | 2.33× | 4.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.083 | 0.87× |
| 1 | 5 | 0.342 | 0.287 | 0.84× |
| 1 | 10 | 0.497 | 0.585 | 1.18× |
| 10 | 1 | 0.048 | 0.056 | 1.16× |
| 10 | 5 | 0.233 | 0.290 | 1.24× |
| 10 | 10 | 0.447 | 0.564 | 1.26× |
| 100 | 1 | 0.051 | 0.062 | 1.22× |
| 100 | 5 | 0.219 | 0.268 | 1.22× |
| 100 | 10 | 0.469 | 0.622 | 1.33× |
| 1,000 | 1 | 0.054 | 0.057 | 1.06× |
| 1,000 | 5 | 0.272 | 0.312 | 1.15× |
| 1,000 | 10 | 0.496 | 0.619 | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
