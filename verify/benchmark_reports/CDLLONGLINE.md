# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.83M | 0.005 | 185.61M | 0.038 | 5.43× | 7.10× |
| 10,000 | 0.090 | 111.20M | 0.088 | 113.60M | 0.184 | 2.05× | 2.09× |
| 100,000 | 1.038 | 96.31M | 1.025 | 97.52M | 1.561 | 1.50× | 1.52× |
| 1,000,000 | 10.723 | 93.26M | 10.792 | 92.66M | 15.520 | 1.45× | 1.44× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.029 ms**; native kernel **1.037 ms**; TA-Lib 1.566 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.353 | 0.290 | 3.45M | 1547.489 | 5339.19× | 92.74× |
| 100,000 | 10 | 2.688 | 1.449 | 6.90M | 1535.682 | 1059.99× | 19.27× |
| 100,000 | 1,000 | 32.257 | 31.295 | 31.95M | 1553.619 | 49.64× | 1.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 81.83M | 85.10M | 1.00× | 2.38M | 2.43M | 1.00× | 59.79M |
| 2 | 170.75M | 178.37M | 2.10× | 2.28M | 2.50M | 1.03× | 60.28M |
| 4 | 283.87M | 325.38M | 3.82× | 2.20M | 2.35M | 0.97× | 59.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
