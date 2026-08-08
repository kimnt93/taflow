# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.51M | 0.016 | 62.83M | 0.043 | 2.35× | 2.71× |
| 10,000 | 0.137 | 72.87M | 0.129 | 77.81M | 0.158 | 1.15× | 1.23× |
| 100,000 | 1.294 | 77.27M | 1.300 | 76.90M | 1.294 | 1.00× | 0.99× |
| 1,000,000 | 15.496 | 64.53M | 14.103 | 70.91M | 13.472 | 0.87× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.297 ms**; native kernel **1.324 ms**; TA-Lib 1.501 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.259 | 0.166 | 6.03M | 1377.454 | 8312.56× | 208.90× |
| 100,000 | 10 | 1.433 | 0.806 | 12.41M | 1369.400 | 1699.19× | 42.49× |
| 100,000 | 1,000 | 23.048 | 24.725 | 40.44M | 1488.991 | 60.22× | 1.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 59.49M | 66.13M | 1.00× | 2.97M | 2.86M | 1.00× | 67.97M |
| 2 | 85.58M | 125.65M | 1.90× | 2.67M | 3.32M | 1.16× | 67.21M |
| 4 | 215.54M | 189.71M | 2.87× | 2.75M | 2.77M | 0.97× | 66.09M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
