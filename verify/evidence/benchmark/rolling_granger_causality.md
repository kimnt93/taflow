# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.875 | 533.31K | 1.880 | 532.00K | 8.650 | 4.61× | 4.60× |
| 10,000 | 20.956 | 477.20K | 20.202 | 495.00K | 83.686 | 3.99× | 4.14× |
| 100,000 | 214.863 | 465.41K | 203.744 | 490.81K | 844.345 | 3.93× | 4.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.281 | 2.23× |
| 1 | 5 | 0.243 | 1.196 | 4.93× |
| 1 | 10 | 0.427 | 2.701 | 6.33× |
| 10 | 1 | 0.043 | 0.282 | 6.54× |
| 10 | 5 | 0.207 | 1.505 | 7.26× |
| 10 | 10 | 0.403 | 2.675 | 6.64× |
| 100 | 1 | 0.140 | 0.764 | 5.46× |
| 100 | 5 | 0.284 | 3.651 | 12.87× |
| 100 | 10 | 0.522 | 6.982 | 13.37× |
| 1,000 | 1 | 2.039 | 8.059 | 3.95× |
| 1,000 | 5 | 3.721 | 41.803 | 11.23× |
| 1,000 | 10 | 4.208 | 83.012 | 19.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
