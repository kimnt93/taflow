# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.33M | 0.009 | 105.63M | 0.040 | 3.57× | 4.27× |
| 10,000 | 0.128 | 78.00M | 0.118 | 84.91M | 0.141 | 1.10× | 1.19× |
| 100,000 | 1.230 | 81.27M | 1.201 | 83.26M | 1.087 | 0.88× | 0.91× |
| 1,000,000 | 13.289 | 75.25M | 12.795 | 78.16M | 10.701 | 0.81× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.231 ms**; native kernel **1.241 ms**; TA-Lib 1.088 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.386 | 0.276 | 3.62M | 1091.106 | 3953.36× | 119.51× |
| 100,000 | 10 | 2.135 | 1.421 | 7.04M | 1126.219 | 792.79× | 23.02× |
| 100,000 | 1,000 | 84.780 | 74.124 | 13.49M | 1103.038 | 14.88× | 0.55× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 62.51M | 72.16M | 1.00× | 1.84M | 2.09M | 1.00× | 76.21M |
| 2 | 111.16M | 140.18M | 1.94× | 1.88M | 1.95M | 0.93× | 75.63M |
| 4 | 175.15M | 251.06M | 3.48× | 1.87M | 1.93M | 0.92× | 75.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
