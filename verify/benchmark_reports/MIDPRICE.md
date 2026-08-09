# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.04M | 0.009 | 117.18M | 0.040 | 3.99× | 4.68× |
| 10,000 | 0.085 | 117.66M | 0.081 | 123.73M | 0.108 | 1.27× | 1.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.388 | 0.237 | 4.23M | 46.689 | 197.41× | 147.31× |
| 1,500 | 10 | 2.077 | 1.064 | 9.40M | 48.063 | 45.17× | 32.22× |
| 1,500 | 100 | 5.902 | 3.711 | 26.95M | 47.986 | 12.93× | 9.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.53M | 12.79M | 1.00× | 1.16M | 1.10M | 1.00× | 8.70M |
| 2 | 16.06M | 19.32M | 1.51× | 1.16M | 1.42M | 1.29× | 8.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
