# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.90M | 0.005 | 207.89M | 0.036 | 5.34× | 7.41× |
| 10,000 | 0.078 | 128.79M | 0.062 | 162.03M | 0.129 | 1.66× | 2.09× |
| 100,000 | 0.922 | 108.51M | 0.892 | 112.05M | 1.058 | 1.15× | 1.19× |
| 1,000,000 | 9.650 | 103.62M | 9.467 | 105.63M | 10.747 | 1.11× | 1.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.938 ms**; native kernel **0.890 ms**; TA-Lib 1.071 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.329 | 0.263 | 3.81M | 1064.009 | 4053.17× | 108.09× |
| 100,000 | 10 | 2.560 | 1.350 | 7.41M | 1074.231 | 795.47× | 20.17× |
| 100,000 | 1,000 | 30.755 | 27.970 | 35.75M | 1060.557 | 37.92× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 81.69M | 88.09M | 1.00× | 2.13M | 2.19M | 1.00× | 76.04M |
| 2 | 170.24M | 178.29M | 2.02× | 2.38M | 2.52M | 1.15× | 78.66M |
| 4 | 264.75M | 344.29M | 3.91× | 2.38M | 2.53M | 1.15× | 78.63M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
