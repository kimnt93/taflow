# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 360.01M | 0.002 | 601.83M | 0.027 | 9.89× | 16.53× |
| 10,000 | 0.014 | 735.94M | 0.011 | 907.18M | 0.043 | 3.19× | 3.93× |
| 100,000 | 0.161 | 620.86M | 0.132 | 758.80M | 0.164 | 1.02× | 1.25× |
| 1,000,000 | 2.786 | 358.96M | 2.243 | 445.86M | 1.530 | 0.55× | 0.68× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.156 ms**; native kernel **0.132 ms**; TA-Lib 0.186 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.236 | 0.156 | 6.41M | 167.478 | 1074.37× | 166.81× |
| 100,000 | 10 | 0.914 | 0.623 | 16.06M | 164.121 | 263.63× | 41.08× |
| 100,000 | 1,000 | 3.752 | 2.380 | 420.24M | 165.338 | 69.48× | 11.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 269.12M | 378.26M | 1.00× | 3.36M | 3.74M | 1.00× | 366.89M |
| 2 | 382.86M | 559.93M | 1.48× | 3.05M | 3.32M | 0.89× | 375.29M |
| 4 | 392.48M | 762.84M | 2.02× | 2.95M | 3.04M | 0.81× | 371.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
