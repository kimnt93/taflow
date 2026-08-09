# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.00M | 0.021 | 48.25M | 0.050 | 2.45× | 2.41× |
| 10,000 | 0.177 | 56.43M | 0.178 | 56.04M | 0.186 | 1.05× | 1.04× |
| 100,000 | 1.710 | 58.50M | 1.684 | 59.38M | 1.536 | 0.90× | 0.91× |
| 1,000,000 | 17.111 | 58.44M | 16.567 | 60.36M | 15.129 | 0.88× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.164 | 1.41× |
| 1 | 5 | 0.327 | 0.532 | 1.63× |
| 1 | 10 | 0.486 | 0.988 | 2.03× |
| 10 | 1 | 0.058 | 0.100 | 1.74× |
| 10 | 5 | 0.235 | 0.489 | 2.08× |
| 10 | 10 | 0.513 | 1.046 | 2.04× |
| 100 | 1 | 0.063 | 0.107 | 1.69× |
| 100 | 5 | 0.246 | 0.480 | 1.95× |
| 100 | 10 | 0.569 | 1.117 | 1.96× |
| 1,000 | 1 | 0.084 | 0.173 | 2.05× |
| 1,000 | 5 | 0.264 | 0.553 | 2.10× |
| 1,000 | 10 | 0.574 | 1.302 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
