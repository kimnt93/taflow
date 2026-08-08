# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.52M | 0.010 | 104.37M | 0.042 | 0.77× | 4.35× |
| 10,000 | 0.566 | 17.68M | 0.068 | 147.40M | 0.060 | 0.11× | 0.89× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.352 | 0.200 | 5.00M | 38.551 | 192.64× | 165.20× |
| 1,500 | 10 | 1.728 | 0.699 | 14.30M | 37.206 | 53.20× | 46.34× |
| 1,500 | 100 | 7.841 | 2.907 | 34.40M | 36.744 | 12.64× | 11.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
