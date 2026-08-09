# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.45M | 0.008 | 133.12M | 0.032 | 3.74× | 4.20× |
| 10,000 | 0.071 | 140.43M | 0.067 | 149.60M | 0.085 | 1.19× | 1.27× |
| 100,000 | 0.677 | 147.74M | 0.662 | 150.95M | 0.606 | 0.90× | 0.92× |
| 1,000,000 | 7.402 | 135.11M | 7.033 | 142.19M | 5.847 | 0.79× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.676 ms**; native kernel **0.655 ms**; TA-Lib 0.627 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.239 | 0.167 | 5.99M | 611.522 | 3664.90× | 150.96× |
| 100,000 | 10 | 0.907 | 0.568 | 17.61M | 606.090 | 1067.26× | 44.17× |
| 100,000 | 1,000 | 9.561 | 10.116 | 98.86M | 608.557 | 60.16× | 3.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.35M | 129.08M | 1.00× | 3.27M | 3.26M | 1.00× | 127.77M |
| 2 | 222.19M | 252.16M | 1.95× | 3.42M | 3.59M | 1.10× | 142.51M |
| 4 | 286.51M | 428.87M | 3.32× | 3.09M | 3.33M | 1.02× | 139.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
