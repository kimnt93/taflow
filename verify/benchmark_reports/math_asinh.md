# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.55M | 0.016 | 62.47M | 0.013 | 0.54× | 0.81× |
| 10,000 | 0.132 | 75.95M | 0.136 | 73.74M | 0.115 | 0.87× | 0.85× |
| 100,000 | 1.263 | 79.15M | 1.159 | 86.26M | 1.059 | 0.84× | 0.91× |
| 1,000,000 | 12.823 | 77.99M | 13.270 | 75.36M | 11.354 | 0.89× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.073 | 0.84× |
| 1 | 5 | 0.319 | 0.239 | 0.75× |
| 1 | 10 | 0.525 | 0.461 | 0.88× |
| 10 | 1 | 0.059 | 0.045 | 0.76× |
| 10 | 5 | 0.286 | 0.199 | 0.69× |
| 10 | 10 | 0.536 | 0.473 | 0.88× |
| 100 | 1 | 0.052 | 0.046 | 0.87× |
| 100 | 5 | 0.277 | 0.284 | 1.03× |
| 100 | 10 | 0.538 | 0.426 | 0.79× |
| 1,000 | 1 | 0.059 | 0.052 | 0.88× |
| 1,000 | 5 | 0.248 | 0.212 | 0.85× |
| 1,000 | 10 | 0.592 | 0.568 | 0.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
