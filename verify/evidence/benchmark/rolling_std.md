# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.42M | 0.006 | 159.97M | 0.036 | 4.83× | 5.74× |
| 10,000 | 0.047 | 211.54M | 0.042 | 237.61M | 0.063 | 1.33× | 1.49× |
| 100,000 | 0.428 | 233.76M | 0.385 | 259.48M | 0.323 | 0.75× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.134 | 0.90× |
| 1 | 5 | 0.299 | 0.508 | 1.70× |
| 1 | 10 | 0.459 | 0.975 | 2.12× |
| 10 | 1 | 0.051 | 0.090 | 1.77× |
| 10 | 5 | 0.241 | 0.466 | 1.93× |
| 10 | 10 | 0.480 | 0.996 | 2.08× |
| 100 | 1 | 0.052 | 0.094 | 1.81× |
| 100 | 5 | 0.257 | 0.471 | 1.83× |
| 100 | 10 | 0.490 | 1.014 | 2.07× |
| 1,000 | 1 | 0.058 | 0.094 | 1.63× |
| 1,000 | 5 | 0.242 | 0.490 | 2.03× |
| 1,000 | 10 | 0.502 | 1.013 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
