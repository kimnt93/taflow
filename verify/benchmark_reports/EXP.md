# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.93M | 0.006 | 176.49M | 0.030 | 4.48× | 5.28× |
| 10,000 | 0.051 | 194.39M | 0.050 | 199.73M | 0.069 | 1.34× | 1.38× |
| 100,000 | 0.508 | 196.76M | 0.488 | 204.73M | 0.449 | 0.88× | 0.92× |
| 1,000,000 | 5.814 | 172.00M | 5.377 | 185.98M | 4.343 | 0.75× | 0.81× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.509 ms**; native kernel **0.484 ms**; TA-Lib 0.454 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.228 | 0.165 | 6.06M | 446.576 | 2705.95× | 149.02× |
| 100,000 | 10 | 0.898 | 0.515 | 19.43M | 448.270 | 870.90× | 48.00× |
| 100,000 | 1,000 | 7.286 | 7.274 | 137.48M | 455.864 | 62.67× | 4.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 165.00M | 171.39M | 1.00× | 3.22M | 3.70M | 1.00× | 179.39M |
| 2 | 261.21M | 313.92M | 1.83× | 3.29M | 3.34M | 0.90× | 171.76M |
| 4 | 356.76M | 498.06M | 2.91× | 2.87M | 3.18M | 0.86× | 170.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
