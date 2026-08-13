# MathRadians benchmark (`numpy.radians` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.73M | 0.019 | 53.11M | 0.013 | 0.54× | 0.69× |
| 10,000 | 0.144 | 69.28M | 0.130 | 76.88M | 0.024 | 0.17× | 0.19× |
| 100,000 | 1.257 | 79.55M | 1.224 | 81.72M | 0.129 | 0.10× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.086 | 0.75× |
| 1 | 5 | 0.361 | 0.288 | 0.80× |
| 1 | 10 | 0.571 | 0.561 | 0.98× |
| 10 | 1 | 0.060 | 0.056 | 0.93× |
| 10 | 5 | 0.261 | 0.262 | 1.01× |
| 10 | 10 | 0.560 | 0.554 | 0.99× |
| 100 | 1 | 0.064 | 0.055 | 0.86× |
| 100 | 5 | 0.276 | 0.264 | 0.96× |
| 100 | 10 | 0.581 | 0.565 | 0.97× |
| 1,000 | 1 | 0.086 | 0.061 | 0.71× |
| 1,000 | 5 | 0.285 | 0.285 | 1.00× |
| 1,000 | 10 | 0.603 | 0.601 | 1.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
