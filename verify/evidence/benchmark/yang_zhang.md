# YangZhang benchmark (`YangZhangVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.96M | 0.050 | 20.05M | 0.320 | 6.07× | 6.42× |
| 10,000 | 0.461 | 21.71M | 0.466 | 21.46M | 1.769 | 3.84× | 3.80× |
| 100,000 | 4.414 | 22.65M | 4.379 | 22.84M | 17.255 | 3.91× | 3.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.290 | 3.44× |
| 1 | 5 | 0.341 | 1.469 | 4.31× |
| 1 | 10 | 0.552 | 2.589 | 4.69× |
| 10 | 1 | 0.059 | 0.251 | 4.27× |
| 10 | 5 | 0.259 | 1.477 | 5.71× |
| 10 | 10 | 0.566 | 2.870 | 5.07× |
| 100 | 1 | 0.071 | 0.264 | 3.71× |
| 100 | 5 | 0.280 | 1.592 | 5.68× |
| 100 | 10 | 0.617 | 2.822 | 4.58× |
| 1,000 | 1 | 0.113 | 0.419 | 3.69× |
| 1,000 | 5 | 0.290 | 2.446 | 8.43× |
| 1,000 | 10 | 0.612 | 4.565 | 7.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
