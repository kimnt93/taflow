# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.63M | 0.006 | 179.51M | 0.031 | 0.65× | 5.64× |
| 10,000 | 0.452 | 22.13M | 0.045 | 220.53M | 0.051 | 0.11× | 1.13× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.008 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.515 | 0.177 | 5.64M | 32.768 | 184.78× | 204.31× |
| 1,500 | 10 | 1.862 | 0.760 | 13.15M | 39.294 | 51.68× | 45.36× |
| 1,500 | 100 | 13.887 | 4.457 | 22.44M | 35.132 | 7.88× | 6.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
