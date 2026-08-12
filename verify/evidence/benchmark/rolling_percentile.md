# RollingPercentile benchmark (`rolling percentile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.25M | 0.044 | 22.69M | 0.359 | 7.63× | 8.15× |
| 10,000 | 0.523 | 19.10M | 0.473 | 21.15M | 2.174 | 4.15× | 4.60× |
| 100,000 | 4.591 | 21.78M | 4.711 | 21.23M | 20.953 | 4.56× | 4.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.238 | 2.02× |
| 1 | 5 | 0.327 | 1.013 | 3.09× |
| 1 | 10 | 0.469 | 2.162 | 4.61× |
| 10 | 1 | 0.049 | 0.207 | 4.21× |
| 10 | 5 | 0.244 | 1.007 | 4.12× |
| 10 | 10 | 0.481 | 2.243 | 4.67× |
| 100 | 1 | 0.059 | 0.257 | 4.36× |
| 100 | 5 | 0.252 | 1.253 | 4.98× |
| 100 | 10 | 0.506 | 2.624 | 5.19× |
| 1,000 | 1 | 0.103 | 0.468 | 4.54× |
| 1,000 | 5 | 0.282 | 1.512 | 5.36× |
| 1,000 | 10 | 0.541 | 3.010 | 5.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
