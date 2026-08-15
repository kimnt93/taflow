# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 258.75M | 0.003 | 332.51M | 0.033 | 8.56× | 10.99× |
| 10,000 | 0.028 | 352.61M | 0.023 | 441.64M | 0.051 | 1.81× | 2.27× |
| 100,000 | 0.244 | 410.36M | 0.223 | 448.83M | 0.239 | 0.98× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.121 | 1.83× |
| 1 | 5 | 0.198 | 0.475 | 2.40× |
| 1 | 10 | 0.394 | 0.924 | 2.35× |
| 10 | 1 | 0.041 | 0.097 | 2.35× |
| 10 | 5 | 0.187 | 0.439 | 2.35× |
| 10 | 10 | 0.376 | 0.946 | 2.52× |
| 100 | 1 | 0.044 | 0.099 | 2.26× |
| 100 | 5 | 0.186 | 0.449 | 2.41× |
| 100 | 10 | 0.380 | 0.958 | 2.52× |
| 1,000 | 1 | 0.044 | 0.104 | 2.37× |
| 1,000 | 5 | 0.206 | 0.489 | 2.37× |
| 1,000 | 10 | 0.422 | 0.956 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
