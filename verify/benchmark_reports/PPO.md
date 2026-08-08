# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.39M | 0.013 | 76.51M | 0.039 | 0.71× | 2.96× |
| 10,000 | 0.514 | 19.46M | 0.118 | 84.40M | 0.092 | 0.18× | 0.77× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.081 ms**; native kernel **0.018 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.291 | 0.189 | 5.29M | 46.916 | 248.34× | 182.71× |
| 1,500 | 10 | 1.744 | 0.721 | 13.88M | 43.470 | 60.32× | 50.07× |
| 1,500 | 100 | 7.437 | 3.191 | 31.34M | 46.354 | 14.53× | 10.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
