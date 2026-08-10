# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.69M | 0.012 | 83.88M | 98.388 | 6561.64× | 8252.56× |
| 10,000 | 0.094 | 106.75M | 0.079 | 126.85M | 965.869 | 10310.47× | 12252.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 1.712 | 13.25× |
| 1 | 5 | 0.333 | 9.730 | 29.21× |
| 1 | 10 | 0.558 | 25.214 | 45.17× |
| 10 | 1 | 0.071 | 3.128 | 44.14× |
| 10 | 5 | 0.267 | 15.296 | 57.38× |
| 10 | 10 | 0.564 | 28.691 | 50.91× |
| 100 | 1 | 0.064 | 11.083 | 172.28× |
| 100 | 5 | 0.319 | 61.710 | 193.57× |
| 100 | 10 | 0.768 | 128.110 | 166.90× |
| 1,000 | 1 | 0.080 | 95.634 | 1202.55× |
| 1,000 | 5 | 0.529 | 590.628 | 1116.38× |
| 1,000 | 10 | 0.701 | 1200.609 | 1712.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
