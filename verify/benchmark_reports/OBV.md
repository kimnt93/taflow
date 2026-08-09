# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.80M | 0.004 | 231.52M | 0.031 | 5.48× | 7.22× |
| 10,000 | 0.043 | 230.95M | 0.039 | 256.07M | 0.066 | 1.52× | 1.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.356 | 0.202 | 4.96M | 31.806 | 157.68× | 147.46× |
| 1,500 | 10 | 1.694 | 0.826 | 12.11M | 32.531 | 39.41× | 36.05× |
| 1,500 | 100 | 3.812 | 2.224 | 44.96M | 34.179 | 15.37× | 13.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.17M | 18.10M | 1.00× | 1.26M | 1.16M | 1.00× | 9.83M |
| 2 | 18.76M | 20.83M | 1.15× | 1.35M | 1.48M | 1.28× | 10.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
