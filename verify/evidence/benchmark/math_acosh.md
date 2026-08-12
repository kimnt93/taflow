# MathAcosh benchmark (`numpy.arccosh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.93M | 0.012 | 85.01M | 0.021 | 1.66× | 1.75× |
| 10,000 | 0.105 | 95.55M | 0.102 | 98.09M | 0.109 | 1.04× | 1.07× |
| 100,000 | 1.078 | 92.74M | 1.096 | 91.24M | 1.055 | 0.98× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.187 | 0.072 | 0.38× |
| 1 | 5 | 0.321 | 0.270 | 0.84× |
| 1 | 10 | 0.453 | 0.565 | 1.25× |
| 10 | 1 | 0.050 | 0.055 | 1.11× |
| 10 | 5 | 0.220 | 0.267 | 1.21× |
| 10 | 10 | 0.516 | 0.654 | 1.27× |
| 100 | 1 | 0.051 | 0.057 | 1.11× |
| 100 | 5 | 0.221 | 0.299 | 1.35× |
| 100 | 10 | 0.497 | 0.630 | 1.27× |
| 1,000 | 1 | 0.063 | 0.071 | 1.12× |
| 1,000 | 5 | 0.272 | 0.394 | 1.45× |
| 1,000 | 10 | 0.505 | 0.765 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
