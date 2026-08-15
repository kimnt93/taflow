# MathAsinh benchmark (`numpy.arcsinh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.63M | 0.013 | 75.53M | 0.025 | 1.73× | 1.85× |
| 10,000 | 0.132 | 75.59M | 0.134 | 74.76M | 0.148 | 1.12× | 1.11× |
| 100,000 | 1.330 | 75.16M | 1.293 | 77.35M | 1.366 | 1.03× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.072 | 0.87× |
| 1 | 5 | 0.283 | 0.314 | 1.11× |
| 1 | 10 | 0.376 | 0.573 | 1.52× |
| 10 | 1 | 0.038 | 0.053 | 1.38× |
| 10 | 5 | 0.180 | 0.266 | 1.48× |
| 10 | 10 | 0.405 | 0.604 | 1.49× |
| 100 | 1 | 0.048 | 0.064 | 1.34× |
| 100 | 5 | 0.186 | 0.278 | 1.49× |
| 100 | 10 | 0.396 | 0.580 | 1.47× |
| 1,000 | 1 | 0.055 | 0.074 | 1.35× |
| 1,000 | 5 | 0.186 | 0.359 | 1.93× |
| 1,000 | 10 | 0.444 | 0.757 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
