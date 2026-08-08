# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.04M | 0.012 | 86.14M | 0.037 | 3.01× | 3.20× |
| 10,000 | 0.157 | 63.54M | 0.160 | 62.52M | 0.198 | 1.26× | 1.24× |
| 100,000 | 1.641 | 60.94M | 1.588 | 62.99M | 1.510 | 0.92× | 0.95× |
| 1,000,000 | 16.881 | 59.24M | 16.610 | 60.21M | 15.298 | 0.91× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.615 ms**; native kernel **1.551 ms**; TA-Lib 1.540 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.258 | 0.167 | 5.99M | 1560.322 | 9352.41× | 147.55× |
| 100,000 | 10 | 1.563 | 0.717 | 13.95M | 1495.772 | 2086.54× | 35.36× |
| 100,000 | 1,000 | 18.772 | 16.780 | 59.60M | 1527.257 | 91.02× | 2.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 54.95M | 57.85M | 1.00× | 2.81M | 3.06M | 1.00× | 60.82M |
| 2 | 104.84M | 115.84M | 2.00× | 2.80M | 3.12M | 1.02× | 58.72M |
| 4 | 148.86M | 204.91M | 3.54× | 2.76M | 2.63M | 0.86× | 57.66M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
