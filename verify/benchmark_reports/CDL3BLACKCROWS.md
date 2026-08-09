# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.63M | 0.004 | 251.84M | 0.030 | 5.19× | 7.45× |
| 10,000 | 0.053 | 186.92M | 0.050 | 200.44M | 0.083 | 1.54× | 1.65× |
| 100,000 | 0.695 | 143.94M | 0.668 | 149.77M | 0.591 | 0.85× | 0.89× |
| 1,000,000 | 7.257 | 137.80M | 7.164 | 139.59M | 5.927 | 0.82× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.679 ms**; native kernel **0.662 ms**; TA-Lib 0.581 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.330 | 0.289 | 3.46M | 582.376 | 2017.80× | 91.57× |
| 100,000 | 10 | 2.536 | 1.326 | 7.54M | 613.783 | 462.90× | 19.97× |
| 100,000 | 1,000 | 23.991 | 20.176 | 49.56M | 583.556 | 28.92× | 1.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 103.04M | 121.66M | 1.00× | 2.33M | 2.61M | 1.00× | 139.63M |
| 2 | 213.80M | 235.72M | 1.94× | 2.40M | 2.45M | 0.94× | 126.90M |
| 4 | 384.63M | 409.38M | 3.36× | 2.32M | 2.39M | 0.92× | 135.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
