# MathAtanh benchmark (`numpy.arctanh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.66M | 0.011 | 87.92M | 0.024 | 2.02× | 2.12× |
| 10,000 | 0.115 | 86.99M | 0.107 | 93.29M | 0.153 | 1.33× | 1.43× |
| 100,000 | 1.115 | 89.69M | 1.087 | 92.02M | 1.266 | 1.14× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.070 | 0.86× |
| 1 | 5 | 0.267 | 0.300 | 1.12× |
| 1 | 10 | 0.384 | 0.555 | 1.45× |
| 10 | 1 | 0.042 | 0.058 | 1.38× |
| 10 | 5 | 0.212 | 0.354 | 1.67× |
| 10 | 10 | 0.404 | 0.570 | 1.41× |
| 100 | 1 | 0.044 | 0.062 | 1.41× |
| 100 | 5 | 0.186 | 0.281 | 1.51× |
| 100 | 10 | 0.412 | 0.635 | 1.54× |
| 1,000 | 1 | 0.057 | 0.077 | 1.35× |
| 1,000 | 5 | 0.217 | 0.315 | 1.45× |
| 1,000 | 10 | 0.387 | 0.733 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
