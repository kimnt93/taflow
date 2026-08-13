# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.04M | 0.030 | 33.15M | 0.029 | 0.82× | 0.96× |
| 10,000 | 0.223 | 44.89M | 0.248 | 40.37M | 0.062 | 0.28× | 0.25× |
| 100,000 | 2.117 | 47.24M | 2.135 | 46.83M | 0.365 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.103 | 0.77× |
| 1 | 5 | 0.385 | 0.485 | 1.26× |
| 1 | 10 | 0.578 | 0.890 | 1.54× |
| 10 | 1 | 0.063 | 0.087 | 1.39× |
| 10 | 5 | 0.285 | 0.421 | 1.48× |
| 10 | 10 | 0.595 | 0.889 | 1.49× |
| 100 | 1 | 0.069 | 0.083 | 1.20× |
| 100 | 5 | 0.294 | 0.412 | 1.40× |
| 100 | 10 | 0.602 | 0.879 | 1.46× |
| 1,000 | 1 | 0.083 | 0.087 | 1.05× |
| 1,000 | 5 | 0.285 | 0.441 | 1.55× |
| 1,000 | 10 | 0.591 | 0.930 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
