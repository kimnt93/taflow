# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.16M | 0.006 | 171.67M | 0.023 | 3.19× | 3.91× |
| 10,000 | 0.037 | 268.58M | 0.031 | 322.88M | 0.043 | 1.15× | 1.38× |
| 100,000 | 0.280 | 357.17M | 0.259 | 386.48M | 0.253 | 0.90× | 0.98× |
| 1,000,000 | 3.170 | 315.48M | 2.695 | 371.12M | 4.434 | 1.40× | 1.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.082 | 0.94× |
| 1 | 5 | 0.338 | 0.355 | 1.05× |
| 1 | 10 | 0.468 | 0.757 | 1.62× |
| 10 | 1 | 0.047 | 0.073 | 1.54× |
| 10 | 5 | 0.221 | 0.362 | 1.64× |
| 10 | 10 | 0.469 | 0.750 | 1.60× |
| 100 | 1 | 0.053 | 0.068 | 1.28× |
| 100 | 5 | 0.237 | 0.358 | 1.51× |
| 100 | 10 | 0.491 | 0.719 | 1.47× |
| 1,000 | 1 | 0.050 | 0.077 | 1.53× |
| 1,000 | 5 | 0.243 | 0.541 | 2.23× |
| 1,000 | 10 | 0.541 | 1.170 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
