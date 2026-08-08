# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.55M | 0.006 | 171.98M | 0.037 | 5.39× | 6.37× |
| 10,000 | 0.055 | 182.14M | 0.054 | 185.15M | 0.087 | 1.59× | 1.62× |
| 100,000 | 0.532 | 187.89M | 0.499 | 200.35M | 0.607 | 1.14× | 1.22× |
| 1,000,000 | 5.458 | 183.21M | 5.277 | 189.49M | 5.877 | 1.08× | 1.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.524 ms**; native kernel **0.503 ms**; TA-Lib 0.605 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.216 | 0.145 | 6.88M | 608.620 | 4184.91× | 216.23× |
| 100,000 | 10 | 0.880 | 0.577 | 17.34M | 619.115 | 1073.44× | 52.76× |
| 100,000 | 1,000 | 7.560 | 6.375 | 156.86M | 610.468 | 95.76× | 5.81× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 127.53M | 158.64M | 1.00× | 2.65M | 3.35M | 1.00× | 138.04M |
| 2 | 258.34M | 309.42M | 1.95× | 2.99M | 3.79M | 1.13× | 135.92M |
| 4 | 417.09M | 533.21M | 3.36× | 3.14M | 3.36M | 1.00× | 131.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
