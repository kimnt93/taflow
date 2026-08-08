# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.53M | 0.028 | 36.32M | 0.034 | 1.09× | 1.22× |
| 10,000 | 0.278 | 35.97M | 0.281 | 35.56M | 0.091 | 0.33× | 0.32× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.044 ms**; native kernel **0.044 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.394 | 0.311 | 3.22M | 36.202 | 116.53× | 88.71× |
| 1,500 | 10 | 2.912 | 1.462 | 6.84M | 35.615 | 24.36× | 19.42× |
| 1,500 | 100 | 8.694 | 5.505 | 18.17M | 39.330 | 7.14× | 5.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
