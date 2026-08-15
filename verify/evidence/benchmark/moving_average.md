# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 249.43M | 0.003 | 328.29M | 0.036 | 9.05× | 11.91× |
| 10,000 | 0.023 | 429.91M | 0.021 | 481.33M | 0.054 | 2.33× | 2.61× |
| 100,000 | 0.223 | 448.46M | 0.191 | 524.75M | 0.229 | 1.03× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.112 | 1.43× |
| 1 | 5 | 0.279 | 0.470 | 1.69× |
| 1 | 10 | 0.384 | 1.002 | 2.61× |
| 10 | 1 | 0.047 | 0.088 | 1.89× |
| 10 | 5 | 0.199 | 0.451 | 2.27× |
| 10 | 10 | 0.391 | 0.941 | 2.41× |
| 100 | 1 | 0.044 | 0.096 | 2.20× |
| 100 | 5 | 0.221 | 0.469 | 2.12× |
| 100 | 10 | 0.381 | 0.941 | 2.47× |
| 1,000 | 1 | 0.043 | 0.093 | 2.14× |
| 1,000 | 5 | 0.194 | 0.446 | 2.30× |
| 1,000 | 10 | 0.462 | 1.000 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
