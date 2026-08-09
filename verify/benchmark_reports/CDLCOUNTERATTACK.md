# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.26M | 0.009 | 114.68M | 0.035 | 3.27× | 4.02× |
| 10,000 | 0.080 | 124.40M | 0.074 | 135.94M | 0.137 | 1.71× | 1.87× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.364 | 0.284 | 3.52M | 39.543 | 139.17× | 103.76× |
| 1,500 | 10 | 2.793 | 1.405 | 7.12M | 40.640 | 28.93× | 20.61× |
| 1,500 | 100 | 6.207 | 3.602 | 27.76M | 40.001 | 11.10× | 8.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.74M | 13.42M | 1.00× | 1.12M | 1.30M | 1.00× | 8.52M |
| 2 | 13.55M | 16.87M | 1.26× | 1.11M | 965.91K | 0.74× | 7.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
