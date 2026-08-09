# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.30M | 0.007 | 149.67M | 0.038 | 4.88× | 5.74× |
| 10,000 | 0.043 | 234.85M | 0.040 | 247.73M | 0.064 | 1.51× | 1.59× |
| 100,000 | 0.396 | 252.31M | 0.372 | 268.91M | 0.256 | 0.65× | 0.69× |
| 1,000,000 | 4.410 | 226.78M | 3.596 | 278.10M | 2.312 | 0.52× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.134 | 2.08× |
| 1 | 5 | 0.283 | 0.492 | 1.74× |
| 1 | 10 | 0.485 | 1.040 | 2.15× |
| 10 | 1 | 0.050 | 0.094 | 1.86× |
| 10 | 5 | 0.225 | 0.460 | 2.04× |
| 10 | 10 | 0.514 | 1.020 | 1.98× |
| 100 | 1 | 0.054 | 0.099 | 1.84× |
| 100 | 5 | 0.243 | 0.480 | 1.98× |
| 100 | 10 | 0.502 | 0.991 | 1.97× |
| 1,000 | 1 | 0.059 | 0.110 | 1.87× |
| 1,000 | 5 | 0.266 | 0.497 | 1.86× |
| 1,000 | 10 | 0.506 | 1.002 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
