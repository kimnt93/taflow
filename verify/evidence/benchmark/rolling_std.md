# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 221.92M | 0.004 | 263.60M | 0.036 | 8.07× | 9.58× |
| 10,000 | 0.031 | 321.04M | 0.027 | 364.07M | 0.060 | 1.91× | 2.17× |
| 100,000 | 0.292 | 342.25M | 0.281 | 356.06M | 0.309 | 1.06× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.119 | 1.12× |
| 1 | 5 | 0.254 | 0.478 | 1.88× |
| 1 | 10 | 0.403 | 1.036 | 2.57× |
| 10 | 1 | 0.045 | 0.098 | 2.17× |
| 10 | 5 | 0.186 | 0.451 | 2.43× |
| 10 | 10 | 0.393 | 0.963 | 2.45× |
| 100 | 1 | 0.045 | 0.104 | 2.33× |
| 100 | 5 | 0.204 | 0.473 | 2.32× |
| 100 | 10 | 0.391 | 0.932 | 2.39× |
| 1,000 | 1 | 0.045 | 0.099 | 2.22× |
| 1,000 | 5 | 0.191 | 0.473 | 2.48× |
| 1,000 | 10 | 0.466 | 0.999 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
