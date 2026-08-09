# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.37M | 0.004 | 243.48M | 0.029 | 5.14× | 7.17× |
| 10,000 | 0.013 | 766.80M | 0.009 | 1.11G | 0.036 | 2.75× | 3.98× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.004 ms**; TA-Lib 0.031 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.360 | 0.245 | 4.09M | 29.682 | 121.31× | 116.30× |
| 1,500 | 10 | 2.056 | 0.994 | 10.06M | 29.951 | 30.13× | 28.45× |
| 1,500 | 100 | 4.005 | 2.240 | 44.65M | 28.956 | 12.93× | 13.03× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.81M | 17.40M | 1.00× | 901.82K | 1.23M | 1.00× | 9.46M |
| 2 | 17.10M | 17.58M | 1.01× | 1.03M | 980.93K | 0.80× | 7.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
