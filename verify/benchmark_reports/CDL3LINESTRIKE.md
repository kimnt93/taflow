# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.20M | 0.005 | 189.19M | 0.032 | 4.66× | 6.12× |
| 10,000 | 0.078 | 127.58M | 0.074 | 135.19M | 0.110 | 1.40× | 1.48× |
| 100,000 | 0.861 | 116.21M | 0.830 | 120.48M | 0.815 | 0.95× | 0.98× |
| 1,000,000 | 9.052 | 110.47M | 8.914 | 112.19M | 8.723 | 0.96× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.872 ms**; native kernel **0.881 ms**; TA-Lib 0.851 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.409 | 0.298 | 3.35M | 852.834 | 2860.19× | 95.91× |
| 100,000 | 10 | 2.819 | 1.557 | 6.42M | 841.745 | 540.78× | 17.85× |
| 100,000 | 1,000 | 39.535 | 36.653 | 27.28M | 930.183 | 25.38× | 0.91× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.97M | 89.60M | 1.00× | 2.29M | 2.40M | 1.00× | 103.28M |
| 2 | 176.60M | 181.56M | 2.03× | 2.22M | 2.33M | 0.97× | 100.47M |
| 4 | 293.78M | 344.47M | 3.84× | 2.21M | 2.43M | 1.01× | 104.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
