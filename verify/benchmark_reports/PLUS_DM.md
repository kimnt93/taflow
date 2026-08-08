# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.00M | 0.006 | 170.08M | 0.036 | 4.87× | 6.04× |
| 10,000 | 0.056 | 177.96M | 0.050 | 198.65M | 0.079 | 1.41× | 1.58× |
| 100,000 | 0.524 | 190.88M | 0.488 | 204.75M | 0.529 | 1.01× | 1.08× |
| 1,000,000 | 5.495 | 182.00M | 5.035 | 198.60M | 4.968 | 0.90× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.529 ms**; native kernel **0.484 ms**; TA-Lib 0.530 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.196 | 5.09M | 512.050 | 2608.71× | 148.90× |
| 100,000 | 10 | 1.561 | 0.803 | 12.46M | 517.299 | 644.43× | 38.14× |
| 100,000 | 1,000 | 8.079 | 6.635 | 150.72M | 527.621 | 79.52× | 5.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.22M | 154.35M | 1.00× | 2.84M | 2.52M | 1.00× | 133.22M |
| 2 | 258.21M | 235.77M | 1.53× | 2.81M | 3.30M | 1.31× | 151.58M |
| 4 | 429.13M | 510.88M | 3.31× | 2.93M | 2.99M | 1.19× | 154.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
