# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.361 | 2.77M | 0.056 | 17.73M | 0.092 | 0.26× | 1.64× |
| 10,000 | 3.535 | 2.83M | 0.573 | 17.44M | 0.563 | 0.16× | 0.98× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.535 ms**; native kernel **0.087 ms**; TA-Lib 0.115 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.406 | 0.839 | 1.19M | 113.397 | 135.17× | 46.87× |
| 1,500 | 10 | 4.548 | 1.340 | 7.47M | 116.127 | 86.69× | 29.00× |
| 1,500 | 100 | 33.795 | 7.870 | 12.71M | 126.971 | 16.13× | 5.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
