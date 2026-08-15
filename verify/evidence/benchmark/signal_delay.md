# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 193.32M | 0.004 | 236.25M | 0.028 | 5.36× | 6.56× |
| 10,000 | 0.036 | 277.93M | 0.034 | 294.00M | 0.028 | 0.77× | 0.82× |
| 100,000 | 0.357 | 280.47M | 0.327 | 305.39M | 0.068 | 0.19× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.093 | 1.23× |
| 1 | 5 | 0.248 | 0.474 | 1.91× |
| 1 | 10 | 0.397 | 0.826 | 2.08× |
| 10 | 1 | 0.044 | 0.094 | 2.13× |
| 10 | 5 | 0.190 | 0.420 | 2.20× |
| 10 | 10 | 0.396 | 0.924 | 2.33× |
| 100 | 1 | 0.046 | 0.154 | 3.35× |
| 100 | 5 | 0.210 | 0.426 | 2.03× |
| 100 | 10 | 0.418 | 0.905 | 2.17× |
| 1,000 | 1 | 0.049 | 0.090 | 1.84× |
| 1,000 | 5 | 0.203 | 0.437 | 2.16× |
| 1,000 | 10 | 0.411 | 0.956 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
