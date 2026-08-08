# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.83M | 0.049 | 20.34M | 0.041 | 0.77× | 0.83× |
| 10,000 | 0.512 | 19.54M | 0.525 | 19.03M | 0.120 | 0.23× | 0.23× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.080 ms**; native kernel **0.074 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.414 | 0.332 | 3.01M | 44.663 | 134.49× | 101.90× |
| 1,500 | 10 | 3.065 | 1.627 | 6.15M | 43.635 | 26.82× | 21.06× |
| 1,500 | 100 | 10.582 | 7.369 | 13.57M | 44.975 | 6.10× | 5.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
