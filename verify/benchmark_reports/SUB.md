# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.272 | 3.67M | 0.001 | 810.71M | 0.032 | 0.12× | 25.87× |
| 10,000 | 2.615 | 3.82M | 0.004 | 2.32G | 0.035 | 0.01× | 8.04× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.396 ms**; native kernel **0.001 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.362 | 0.216 | 4.63M | 29.412 | 136.12× | 132.85× |
| 1,500 | 10 | 8.132 | 4.724 | 2.12M | 28.973 | 6.13× | 6.06× |
| 1,500 | 100 | 28.287 | 2.499 | 40.02M | 33.331 | 13.34× | 11.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
