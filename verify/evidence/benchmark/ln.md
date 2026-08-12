# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.36M | 0.007 | 147.97M | 0.031 | 4.07× | 4.62× |
| 10,000 | 0.047 | 214.26M | 0.043 | 231.70M | 0.067 | 1.43× | 1.55× |
| 100,000 | 0.442 | 226.24M | 0.408 | 245.29M | 0.426 | 0.96× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.110 | 1.06× |
| 1 | 5 | 0.235 | 0.429 | 1.83× |
| 1 | 10 | 0.483 | 0.938 | 1.94× |
| 10 | 1 | 0.053 | 0.081 | 1.55× |
| 10 | 5 | 0.224 | 0.414 | 1.85× |
| 10 | 10 | 0.477 | 0.918 | 1.93× |
| 100 | 1 | 0.057 | 0.098 | 1.71× |
| 100 | 5 | 0.219 | 0.410 | 1.87× |
| 100 | 10 | 0.483 | 0.888 | 1.84× |
| 1,000 | 1 | 0.056 | 0.090 | 1.60× |
| 1,000 | 5 | 0.266 | 0.481 | 1.81× |
| 1,000 | 10 | 0.499 | 0.913 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
