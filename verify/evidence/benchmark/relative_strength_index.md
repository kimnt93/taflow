# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.29M | 0.008 | 123.89M | 0.039 | 4.21× | 4.87× |
| 10,000 | 0.078 | 128.83M | 0.075 | 132.66M | 0.088 | 1.13× | 1.16× |
| 100,000 | 0.749 | 133.56M | 0.737 | 135.69M | 0.591 | 0.79× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.127 | 1.90× |
| 1 | 5 | 0.264 | 0.462 | 1.75× |
| 1 | 10 | 0.427 | 1.015 | 2.38× |
| 10 | 1 | 0.048 | 0.107 | 2.22× |
| 10 | 5 | 0.199 | 0.451 | 2.27× |
| 10 | 10 | 0.416 | 0.947 | 2.28× |
| 100 | 1 | 0.047 | 0.097 | 2.06× |
| 100 | 5 | 0.193 | 0.490 | 2.54× |
| 100 | 10 | 0.411 | 0.964 | 2.35× |
| 1,000 | 1 | 0.047 | 0.097 | 2.06× |
| 1,000 | 5 | 0.199 | 0.505 | 2.54× |
| 1,000 | 10 | 0.445 | 1.081 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
