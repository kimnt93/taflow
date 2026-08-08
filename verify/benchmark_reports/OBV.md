# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 287.30M | 0.002 | 526.50M | 0.031 | 8.93× | 16.36× |
| 10,000 | 0.029 | 341.26M | 0.029 | 349.29M | 0.067 | 2.28× | 2.33× |
| 100,000 | 0.459 | 217.69M | 0.428 | 233.49M | 0.391 | 0.85× | 0.91× |
| 1,000,000 | 5.149 | 194.20M | 4.537 | 220.43M | 3.748 | 0.73× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.466 ms**; native kernel **0.438 ms**; TA-Lib 0.388 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.282 | 0.198 | 5.06M | 385.379 | 1948.21× | 144.03× |
| 100,000 | 10 | 1.609 | 0.884 | 11.31M | 390.464 | 441.48× | 32.27× |
| 100,000 | 1,000 | 12.161 | 5.881 | 170.05M | 407.670 | 69.32× | 5.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 140.65M | 195.68M | 1.00× | 2.47M | 2.98M | 1.00× | 176.28M |
| 2 | 295.66M | 330.36M | 1.69× | 2.54M | 3.10M | 1.04× | 206.83M |
| 4 | 404.55M | 615.63M | 3.15× | 2.25M | 2.70M | 0.90× | 189.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
