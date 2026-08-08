# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 261.08M | 0.003 | 357.84M | 0.030 | 7.80× | 10.69× |
| 10,000 | 0.022 | 446.93M | 0.021 | 471.34M | 0.039 | 1.74× | 1.84× |
| 100,000 | 0.200 | 498.87M | 0.176 | 567.53M | 0.125 | 0.62× | 0.71× |
| 1,000,000 | 2.216 | 451.28M | 1.873 | 533.78M | 1.100 | 0.50× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.203 ms**; native kernel **0.176 ms**; TA-Lib 0.124 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.237 | 0.167 | 5.99M | 125.040 | 749.57× | 164.84× |
| 100,000 | 10 | 0.923 | 0.514 | 19.45M | 122.767 | 238.74× | 54.15× |
| 100,000 | 1,000 | 4.304 | 3.073 | 325.38M | 126.573 | 41.19× | 9.48× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 286.29M | 349.73M | 1.00× | 3.43M | 3.62M | 1.00× | 487.09M |
| 2 | 526.16M | 618.99M | 1.77× | 3.13M | 3.89M | 1.07× | 413.44M |
| 4 | 740.34M | 1.20G | 3.44× | 2.97M | 3.33M | 0.92× | 488.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
