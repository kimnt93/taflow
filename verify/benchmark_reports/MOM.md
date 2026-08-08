# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 270.91M | 0.003 | 371.56M | 0.029 | 7.82× | 10.73× |
| 10,000 | 0.022 | 459.15M | 0.021 | 485.42M | 0.033 | 1.53× | 1.61× |
| 100,000 | 0.199 | 503.60M | 0.175 | 569.84M | 0.059 | 0.30× | 0.34× |
| 1,000,000 | 2.246 | 445.14M | 1.877 | 532.77M | 0.618 | 0.28× | 0.33× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.199 ms**; native kernel **0.175 ms**; TA-Lib 0.061 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.174 | 5.75M | 62.974 | 362.04× | 167.59× |
| 100,000 | 10 | 0.906 | 0.569 | 17.57M | 62.146 | 109.22× | 52.67× |
| 100,000 | 1,000 | 4.246 | 3.081 | 324.60M | 63.639 | 20.66× | 9.93× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 295.55M | 402.51M | 1.00× | 3.20M | 4.03M | 1.00× | 711.45M |
| 2 | 586.09M | 746.25M | 1.85× | 3.22M | 3.86M | 0.96× | 653.61M |
| 4 | 701.77M | 1.19G | 2.96× | 3.09M | 3.38M | 0.84× | 618.19M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
