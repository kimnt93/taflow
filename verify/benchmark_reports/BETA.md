# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.10M | 0.006 | 176.49M | 0.038 | 5.36× | 6.70× |
| 10,000 | 0.053 | 187.72M | 0.048 | 208.99M | 0.087 | 1.64× | 1.82× |
| 100,000 | 0.484 | 206.45M | 0.480 | 208.36M | 0.550 | 1.14× | 1.15× |
| 1,000,000 | 5.216 | 191.72M | 4.820 | 207.45M | 5.485 | 1.05× | 1.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.495 ms**; native kernel **0.457 ms**; TA-Lib 0.547 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.291 | 0.176 | 5.69M | 554.824 | 3156.65× | 176.76× |
| 100,000 | 10 | 1.466 | 0.770 | 12.99M | 543.940 | 706.43× | 39.99× |
| 100,000 | 1,000 | 7.710 | 6.399 | 156.28M | 540.772 | 84.51× | 5.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 138.82M | 168.93M | 1.00× | 2.56M | 3.19M | 1.00× | 134.86M |
| 2 | 287.18M | 330.17M | 1.95× | 2.68M | 3.26M | 1.02× | 145.78M |
| 4 | 426.35M | 563.82M | 3.34× | 2.58M | 2.84M | 0.89× | 139.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
