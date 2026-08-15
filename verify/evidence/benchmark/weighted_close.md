# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 339.88M | 0.001 | 787.84M | 0.031 | 10.54× | 24.44× |
| 10,000 | 0.009 | 1.11G | 0.005 | 1.84G | 0.036 | 3.96× | 6.56× |
| 100,000 | 0.073 | 1.36G | 0.050 | 2.01G | 0.086 | 1.17× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.107 | 1.23× |
| 1 | 5 | 0.237 | 0.451 | 1.91× |
| 1 | 10 | 0.435 | 0.918 | 2.11× |
| 10 | 1 | 0.047 | 0.087 | 1.84× |
| 10 | 5 | 0.177 | 0.418 | 2.36× |
| 10 | 10 | 0.380 | 1.004 | 2.64× |
| 100 | 1 | 0.045 | 0.093 | 2.05× |
| 100 | 5 | 0.190 | 0.438 | 2.31× |
| 100 | 10 | 0.383 | 0.903 | 2.35× |
| 1,000 | 1 | 0.043 | 0.095 | 2.20× |
| 1,000 | 5 | 0.188 | 0.470 | 2.51× |
| 1,000 | 10 | 0.387 | 0.914 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
