# MathCot benchmark (`numpy.tan reciprocal` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.68M | 0.011 | 93.55M | 0.027 | 2.20× | 2.52× |
| 10,000 | 0.121 | 82.53M | 0.120 | 83.64M | 0.136 | 1.12× | 1.14× |
| 100,000 | 1.245 | 80.31M | 1.200 | 83.36M | 1.228 | 0.99× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.073 | 0.86× |
| 1 | 5 | 0.220 | 0.284 | 1.29× |
| 1 | 10 | 0.371 | 0.587 | 1.58× |
| 10 | 1 | 0.042 | 0.063 | 1.49× |
| 10 | 5 | 0.193 | 0.304 | 1.58× |
| 10 | 10 | 0.382 | 0.593 | 1.55× |
| 100 | 1 | 0.045 | 0.062 | 1.38× |
| 100 | 5 | 0.193 | 0.295 | 1.53× |
| 100 | 10 | 0.416 | 0.628 | 1.51× |
| 1,000 | 1 | 0.058 | 0.076 | 1.32× |
| 1,000 | 5 | 0.196 | 0.355 | 1.81× |
| 1,000 | 10 | 0.422 | 0.796 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
