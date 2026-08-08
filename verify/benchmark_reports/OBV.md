# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.21M | 0.002 | 508.85M | 0.030 | 0.70× | 15.42× |
| 10,000 | 0.440 | 22.72M | 0.032 | 314.71M | 0.066 | 0.15× | 2.09× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.064 ms**; native kernel **0.002 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.340 | 0.571 | 1.75M | 32.620 | 57.10× | 51.99× |
| 1,500 | 10 | 2.234 | 0.871 | 11.48M | 31.535 | 36.21× | 34.95× |
| 1,500 | 100 | 7.093 | 2.604 | 38.40M | 32.326 | 12.41× | 11.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
