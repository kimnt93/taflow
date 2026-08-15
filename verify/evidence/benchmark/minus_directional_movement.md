# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.70M | 0.006 | 164.76M | 0.040 | 5.59× | 6.59× |
| 10,000 | 0.055 | 181.75M | 0.051 | 196.92M | 0.083 | 1.51× | 1.64× |
| 100,000 | 0.537 | 186.30M | 0.494 | 202.39M | 0.572 | 1.07× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.143 | 1.52× |
| 1 | 5 | 0.245 | 0.485 | 1.98× |
| 1 | 10 | 0.455 | 0.994 | 2.18× |
| 10 | 1 | 0.046 | 0.093 | 2.03× |
| 10 | 5 | 0.246 | 0.625 | 2.54× |
| 10 | 10 | 0.428 | 1.065 | 2.49× |
| 100 | 1 | 0.042 | 0.092 | 2.21× |
| 100 | 5 | 0.186 | 0.452 | 2.44× |
| 100 | 10 | 0.389 | 0.973 | 2.50× |
| 1,000 | 1 | 0.051 | 0.122 | 2.38× |
| 1,000 | 5 | 0.195 | 0.497 | 2.54× |
| 1,000 | 10 | 0.399 | 1.019 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
