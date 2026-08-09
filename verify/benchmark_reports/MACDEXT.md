# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.30M | 0.011 | 90.62M | 0.054 | 4.35× | 4.85× |
| 10,000 | 0.109 | 92.15M | 0.094 | 106.53M | 0.111 | 1.03× | 1.19× |
| 100,000 | 0.965 | 103.58M | 0.936 | 106.78M | 0.693 | 0.72× | 0.74× |
| 1,000,000 | 20.745 | 48.20M | 9.009 | 111.00M | 15.218 | 0.73× | 1.69× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.968 ms**; native kernel **0.891 ms**; TA-Lib 0.699 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.309 | 0.277 | 3.61M | 736.774 | 2660.40× | 165.90× |
| 100,000 | 10 | 1.553 | 1.337 | 7.48M | 692.293 | 517.89× | 34.51× |
| 100,000 | 1,000 | 95.177 | 89.805 | 11.14M | 703.880 | 7.84× | 0.63× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 79.94M | 90.88M | 1.00× | 1.89M | 1.89M | 1.00× | 105.19M |
| 2 | 143.24M | 166.88M | 1.84× | 1.70M | 1.63M | 0.86× | 103.00M |
| 4 | 190.60M | 333.16M | 3.67× | 1.36M | 1.29M | 0.68× | 100.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
