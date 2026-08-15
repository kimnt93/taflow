# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.32M | 0.006 | 173.19M | 0.203 | 27.62× | 35.09× |
| 10,000 | 0.051 | 195.35M | 0.049 | 205.99M | 0.767 | 14.99× | 15.81× |
| 100,000 | 0.470 | 212.73M | 0.449 | 222.64M | 6.538 | 13.91× | 14.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.318 | 3.87× |
| 1 | 5 | 0.249 | 1.062 | 4.27× |
| 1 | 10 | 0.405 | 2.269 | 5.60× |
| 10 | 1 | 0.044 | 0.214 | 4.84× |
| 10 | 5 | 0.192 | 1.229 | 6.42× |
| 10 | 10 | 0.401 | 2.306 | 5.75× |
| 100 | 1 | 0.048 | 0.223 | 4.65× |
| 100 | 5 | 0.198 | 1.247 | 6.30× |
| 100 | 10 | 0.452 | 2.374 | 5.26× |
| 1,000 | 1 | 0.054 | 0.281 | 5.20× |
| 1,000 | 5 | 0.207 | 1.618 | 7.81× |
| 1,000 | 10 | 0.429 | 2.909 | 6.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
