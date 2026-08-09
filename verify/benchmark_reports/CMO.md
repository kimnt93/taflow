# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.21M | 0.007 | 136.23M | 0.036 | 4.15× | 4.95× |
| 10,000 | 0.057 | 176.06M | 0.057 | 174.42M | 0.091 | 1.60× | 1.58× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.011 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.294 | 0.165 | 6.07M | 41.121 | 249.75× | 188.41× |
| 1,500 | 10 | 1.116 | 0.600 | 16.66M | 39.581 | 65.95× | 51.49× |
| 1,500 | 100 | 3.256 | 2.147 | 46.58M | 43.461 | 20.25× | 15.15× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.17M | 16.19M | 1.00× | 1.11M | 1.36M | 1.00× | 9.49M |
| 2 | 15.63M | 22.51M | 1.39× | 1.33M | 1.76M | 1.30× | 9.14M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
