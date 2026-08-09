# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.39M | 0.005 | 205.09M | 0.027 | 4.07× | 5.58× |
| 10,000 | 0.019 | 537.62M | 0.017 | 603.60M | 0.033 | 1.77× | 1.99× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**; TA-Lib 0.027 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.613 | 0.232 | 4.31M | 27.236 | 117.27× | 110.54× |
| 1,500 | 10 | 2.112 | 1.002 | 9.98M | 28.607 | 28.56× | 26.01× |
| 1,500 | 100 | 3.949 | 2.219 | 45.07M | 28.257 | 12.73× | 11.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.70M | 12.97M | 1.00× | 742.82K | 797.17K | 1.00× | 9.24M |
| 2 | 16.36M | 20.51M | 1.58× | 958.78K | 1.54M | 1.93× | 11.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
