# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.99M | 0.008 | 119.38M | 0.038 | 3.74× | 4.56× |
| 10,000 | 0.095 | 105.78M | 0.090 | 111.29M | 0.133 | 1.40× | 1.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.012 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.461 | 0.276 | 3.62M | 44.399 | 160.87× | 103.74× |
| 1,500 | 10 | 2.533 | 1.416 | 7.06M | 43.824 | 30.95× | 20.87× |
| 1,500 | 100 | 6.276 | 3.680 | 27.17M | 46.214 | 12.56× | 7.78× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.50M | 9.93M | 1.00× | 931.62K | 754.14K | 1.00× | 9.35M |
| 2 | 15.19M | 12.55M | 1.26× | 1.31M | 1.10M | 1.46× | 9.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
