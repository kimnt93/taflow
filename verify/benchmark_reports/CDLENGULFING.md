# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.84M | 0.004 | 224.66M | 0.030 | 4.79× | 6.73× |
| 10,000 | 0.074 | 135.03M | 0.073 | 137.80M | 0.088 | 1.19× | 1.22× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.365 | 0.280 | 3.57M | 32.407 | 115.63× | 101.41× |
| 1,500 | 10 | 2.604 | 1.167 | 8.57M | 32.945 | 28.23× | 23.88× |
| 1,500 | 100 | 9.269 | 3.459 | 28.91M | 34.166 | 9.88× | 8.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
