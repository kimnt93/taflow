# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 244.30M | 0.003 | 304.91M | 0.034 | 8.37× | 10.45× |
| 10,000 | 0.033 | 304.47M | 0.030 | 337.11M | 0.078 | 2.39× | 2.64× |
| 100,000 | 0.339 | 295.36M | 0.318 | 314.31M | 0.507 | 1.50× | 1.59× |
| 1,000,000 | 3.857 | 259.26M | 3.439 | 290.74M | 4.709 | 1.22× | 1.37× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.342 ms**; native kernel **0.318 ms**; TA-Lib 0.516 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.249 | 0.176 | 5.67M | 493.013 | 2793.35× | 168.95× |
| 100,000 | 10 | 0.948 | 0.614 | 16.28M | 494.183 | 804.43× | 49.83× |
| 100,000 | 1,000 | 18.484 | 18.603 | 53.76M | 497.500 | 26.74× | 1.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 178.43M | 236.36M | 1.00× | 2.90M | 3.67M | 1.00× | 148.37M |
| 2 | 323.74M | 371.63M | 1.57× | 3.22M | 3.77M | 1.03× | 152.18M |
| 4 | 540.28M | 773.87M | 3.27× | 3.17M | 3.39M | 0.92× | 158.37M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
