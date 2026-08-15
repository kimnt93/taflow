# RollingInformationRatio benchmark (`InformationRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.55M | 0.033 | 30.59M | 0.209 | 6.19× | 6.40× |
| 10,000 | 0.324 | 30.86M | 0.313 | 31.90M | 0.798 | 2.46× | 2.55× |
| 100,000 | 3.198 | 31.27M | 3.249 | 30.78M | 7.051 | 2.20× | 2.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.222 | 3.04× |
| 1 | 5 | 0.258 | 1.058 | 4.10× |
| 1 | 10 | 0.420 | 2.266 | 5.40× |
| 10 | 1 | 0.043 | 0.197 | 4.59× |
| 10 | 5 | 0.206 | 1.027 | 4.98× |
| 10 | 10 | 0.394 | 2.417 | 6.13× |
| 100 | 1 | 0.060 | 0.224 | 3.73× |
| 100 | 5 | 0.224 | 1.101 | 4.91× |
| 100 | 10 | 0.422 | 2.480 | 5.88× |
| 1,000 | 1 | 0.078 | 0.276 | 3.55× |
| 1,000 | 5 | 0.210 | 1.340 | 6.37× |
| 1,000 | 10 | 0.431 | 3.052 | 7.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
