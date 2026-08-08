# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.40M | 0.003 | 382.37M | 0.029 | 0.63× | 11.19× |
| 10,000 | 0.442 | 22.62M | 0.012 | 854.64M | 0.039 | 0.09× | 3.32× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.003 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.400 | 0.286 | 3.50M | 28.035 | 98.11× | 90.69× |
| 1,500 | 10 | 3.086 | 1.270 | 7.87M | 29.460 | 23.19× | 21.92× |
| 1,500 | 100 | 9.407 | 2.873 | 34.81M | 30.927 | 10.76× | 9.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
