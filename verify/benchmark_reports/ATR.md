# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.72M | 0.006 | 159.76M | 0.039 | 4.84× | 6.30× |
| 10,000 | 0.059 | 170.57M | 0.054 | 186.02M | 0.092 | 1.56× | 1.70× |
| 100,000 | 0.544 | 183.99M | 0.521 | 192.04M | 0.623 | 1.15× | 1.20× |
| 1,000,000 | 5.817 | 171.92M | 5.413 | 184.73M | 6.351 | 1.09× | 1.17× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.539 ms**; native kernel **0.502 ms**; TA-Lib 0.618 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.391 | 0.232 | 4.31M | 672.995 | 2898.27× | 139.63× |
| 100,000 | 10 | 1.954 | 1.030 | 9.71M | 644.513 | 626.02× | 31.12× |
| 100,000 | 1,000 | 8.606 | 7.293 | 137.11M | 591.772 | 81.14× | 5.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 138.48M | 147.49M | 1.00× | 2.01M | 2.50M | 1.00× | 99.56M |
| 2 | 251.53M | 283.28M | 1.92× | 2.18M | 2.75M | 1.10× | 123.30M |
| 4 | 418.28M | 469.86M | 3.19× | 2.35M | 2.26M | 0.90× | 108.81M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
