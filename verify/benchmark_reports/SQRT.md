# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.40M | 0.004 | 239.87M | 0.035 | 6.33× | 8.33× |
| 10,000 | 0.024 | 410.16M | 0.023 | 437.97M | 0.040 | 1.63× | 1.74× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**; TA-Lib 0.028 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.281 | 0.176 | 5.70M | 28.040 | 159.71× | 143.38× |
| 1,500 | 10 | 1.088 | 0.575 | 17.38M | 28.304 | 49.19× | 43.83× |
| 1,500 | 100 | 2.875 | 1.762 | 56.75M | 29.786 | 16.90× | 14.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.05M | 9.57M | 1.00× | 1.32M | 1.41M | 1.00× | 10.05M |
| 2 | 16.48M | 21.05M | 2.20× | 1.44M | 1.73M | 1.22× | 10.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
