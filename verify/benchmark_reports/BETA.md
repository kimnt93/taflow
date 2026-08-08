# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.32M | 0.006 | 157.58M | 0.038 | 4.63× | 5.97× |
| 10,000 | 0.060 | 167.44M | 0.056 | 178.29M | 0.085 | 1.43× | 1.52× |
| 100,000 | 0.564 | 177.45M | 0.563 | 177.73M | 0.560 | 0.99× | 1.00× |
| 1,000,000 | 5.989 | 166.97M | 5.615 | 178.10M | 5.434 | 0.91× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.569 ms**; native kernel **0.536 ms**; TA-Lib 0.566 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.271 | 0.175 | 5.72M | 563.937 | 3228.32× | 180.19× |
| 100,000 | 10 | 1.491 | 0.875 | 11.43M | 556.274 | 635.62× | 36.53× |
| 100,000 | 1,000 | 8.541 | 6.897 | 144.99M | 549.184 | 79.63× | 5.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 129.22M | 155.09M | 1.00× | 2.87M | 3.14M | 1.00× | 137.55M |
| 2 | 255.18M | 296.85M | 1.91× | 2.76M | 3.42M | 1.09× | 143.81M |
| 4 | 405.88M | 489.51M | 3.16× | 2.59M | 3.01M | 0.96× | 144.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
