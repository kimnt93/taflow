# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.88M | 0.004 | 248.97M | 0.033 | 0.71× | 8.14× |
| 10,000 | 0.450 | 22.21M | 0.031 | 318.26M | 0.042 | 0.09× | 1.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.006 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.336 | 0.180 | 5.54M | 32.461 | 179.94× | 162.11× |
| 1,500 | 10 | 1.734 | 0.741 | 13.49M | 31.686 | 42.74× | 43.49× |
| 1,500 | 100 | 11.149 | 2.431 | 41.14M | 31.767 | 13.07× | 13.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
