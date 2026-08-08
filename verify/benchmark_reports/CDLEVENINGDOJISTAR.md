# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.22M | 0.005 | 190.43M | 0.039 | 5.66× | 7.42× |
| 10,000 | 0.086 | 116.96M | 0.087 | 115.11M | 0.119 | 1.39× | 1.37× |
| 100,000 | 0.893 | 111.98M | 0.865 | 115.57M | 0.870 | 0.97× | 1.01× |
| 1,000,000 | 9.652 | 103.60M | 9.427 | 106.08M | 8.983 | 0.93× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.957 ms**; native kernel **0.864 ms**; TA-Lib 0.879 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.359 | 0.271 | 3.69M | 883.609 | 3259.87× | 116.41× |
| 100,000 | 10 | 2.816 | 1.358 | 7.36M | 846.898 | 623.47× | 23.56× |
| 100,000 | 1,000 | 36.441 | 31.551 | 31.69M | 862.547 | 27.34× | 1.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 96.42M | 66.69M | 1.00× | 2.10M | 2.50M | 1.00× | 91.37M |
| 2 | 183.98M | 167.51M | 2.51× | 2.17M | 2.76M | 1.10× | 92.05M |
| 4 | 300.26M | 325.56M | 4.88× | 2.28M | 2.42M | 0.97× | 92.54M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
