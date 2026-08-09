# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.56M | 0.007 | 149.93M | 0.034 | 4.00× | 5.14× |
| 10,000 | 0.060 | 166.26M | 0.056 | 177.59M | 0.105 | 1.74× | 1.86× |
| 100,000 | 0.564 | 177.33M | 0.541 | 184.72M | 0.851 | 1.51× | 1.57× |
| 1,000,000 | 6.579 | 152.00M | 6.297 | 158.79M | 8.721 | 1.33× | 1.38× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.565 ms**; native kernel **0.534 ms**; TA-Lib 0.846 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.444 | 0.272 | 3.68M | 1001.871 | 3684.44× | 117.47× |
| 100,000 | 10 | 2.964 | 1.523 | 6.56M | 863.894 | 567.05× | 20.25× |
| 100,000 | 1,000 | 34.564 | 32.457 | 30.81M | 856.570 | 26.39× | 1.09× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 125.45M | 143.50M | 1.00× | 1.94M | 2.33M | 1.00× | 102.13M |
| 2 | 224.29M | 268.04M | 1.87× | 1.88M | 2.40M | 1.03× | 95.36M |
| 4 | 316.88M | 421.08M | 2.93× | 1.85M | 2.12M | 0.91× | 97.51M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
