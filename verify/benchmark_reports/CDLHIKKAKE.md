# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.56M | 0.003 | 295.31M | 0.031 | 5.91× | 9.11× |
| 10,000 | 0.054 | 186.23M | 0.050 | 199.95M | 0.073 | 1.35× | 1.45× |
| 100,000 | 0.567 | 176.52M | 0.549 | 182.02M | 0.470 | 0.83× | 0.86× |
| 1,000,000 | 6.276 | 159.33M | 5.766 | 173.44M | 4.684 | 0.75× | 0.81× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.564 ms**; native kernel **0.550 ms**; TA-Lib 0.470 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.324 | 0.276 | 3.63M | 472.733 | 1715.18× | 98.18× |
| 100,000 | 10 | 2.541 | 1.332 | 7.51M | 470.921 | 353.62× | 20.69× |
| 100,000 | 1,000 | 22.995 | 19.660 | 50.86M | 471.788 | 24.00× | 1.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 120.92M | 134.55M | 1.00× | 2.17M | 2.49M | 1.00× | 155.23M |
| 2 | 290.33M | 279.77M | 2.08× | 2.54M | 2.70M | 1.08× | 167.44M |
| 4 | 433.04M | 516.38M | 3.84× | 2.49M | 2.61M | 1.05× | 157.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
