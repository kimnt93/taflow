# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.88M | 0.008 | 120.07M | 0.036 | 3.52× | 4.28× |
| 10,000 | 0.122 | 81.75M | 0.115 | 87.06M | 0.163 | 1.33× | 1.42× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.446 | 0.286 | 3.49M | 43.459 | 151.81× | 106.76× |
| 1,500 | 10 | 2.739 | 1.372 | 7.29M | 44.458 | 32.41× | 21.02× |
| 1,500 | 100 | 5.744 | 3.511 | 28.48M | 43.944 | 12.52× | 8.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.11M | 14.28M | 1.00× | 909.55K | 952.69K | 1.00× | 6.77M |
| 2 | 16.60M | 18.06M | 1.27× | 1.27M | 1.38M | 1.45× | 8.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
