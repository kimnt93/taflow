# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.47M | 0.021 | 48.71M | 0.045 | 2.00× | 2.19× |
| 10,000 | 0.286 | 34.97M | 0.280 | 35.75M | 0.156 | 0.54× | 0.56× |
| 100,000 | 2.932 | 34.11M | 2.945 | 33.95M | 1.272 | 0.43× | 0.43× |
| 1,000,000 | 35.199 | 28.41M | 34.552 | 28.94M | 12.565 | 0.36× | 0.36× |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.019 ms**; native kernel **2.950 ms**; TA-Lib 1.272 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.386 | 0.286 | 3.50M | 1269.951 | 4441.89× | 123.56× |
| 100,000 | 10 | 1.966 | 1.552 | 6.44M | 1295.914 | 835.07× | 22.90× |
| 100,000 | 1,000 | 69.979 | 71.814 | 13.92M | 1314.514 | 18.30× | 0.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 31.73M | 32.05M | 1.00× | 2.02M | 2.32M | 1.00× | 69.01M |
| 2 | 57.17M | 62.59M | 1.95× | 1.89M | 2.04M | 0.88× | 70.58M |
| 4 | 97.24M | 104.30M | 3.25× | 1.47M | 1.70M | 0.73× | 66.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
