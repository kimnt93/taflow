# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.84M | 0.004 | 235.79M | 0.034 | 5.34× | 7.93× |
| 10,000 | 0.046 | 216.84M | 0.041 | 242.02M | 0.091 | 1.97× | 2.20× |
| 100,000 | 0.576 | 173.48M | 0.555 | 180.20M | 0.634 | 1.10× | 1.14× |
| 1,000,000 | 6.302 | 158.69M | 6.095 | 164.07M | 6.374 | 1.01× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.574 ms**; native kernel **0.561 ms**; TA-Lib 0.642 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.286 | 3.50M | 642.322 | 2246.93× | 93.51× |
| 100,000 | 10 | 2.673 | 1.400 | 7.14M | 639.151 | 456.55× | 20.70× |
| 100,000 | 1,000 | 25.818 | 23.717 | 42.16M | 653.678 | 27.56× | 1.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.98M | 153.97M | 1.00× | 2.55M | 2.57M | 1.00× | 145.93M |
| 2 | 265.88M | 285.93M | 1.86× | 2.24M | 2.75M | 1.07× | 132.95M |
| 4 | 438.07M | 537.12M | 3.49× | 2.33M | 2.45M | 0.95× | 130.51M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
