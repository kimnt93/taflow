# TripleTopBottom benchmark (`TripleTopBottom` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.79M | 0.007 | 139.26M | 0.238 | 23.02× | 33.12× |
| 10,000 | 0.094 | 106.79M | 0.100 | 99.67M | 1.391 | 14.85× | 13.86× |
| 100,000 | 0.900 | 111.06M | 0.863 | 115.85M | 13.972 | 15.52× | 16.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.274 | 2.31× |
| 1 | 5 | 0.240 | 0.869 | 3.62× |
| 1 | 10 | 0.409 | 1.636 | 4.00× |
| 10 | 1 | 0.044 | 0.193 | 4.36× |
| 10 | 5 | 0.226 | 1.131 | 5.00× |
| 10 | 10 | 0.422 | 1.646 | 3.90× |
| 100 | 1 | 0.049 | 0.188 | 3.86× |
| 100 | 5 | 0.236 | 1.198 | 5.09× |
| 100 | 10 | 0.387 | 1.807 | 4.67× |
| 1,000 | 1 | 0.061 | 0.328 | 5.34× |
| 1,000 | 5 | 0.207 | 1.812 | 8.76× |
| 1,000 | 10 | 0.437 | 3.070 | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
