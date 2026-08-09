# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.16M | 0.013 | 79.74M | 0.041 | 3.12× | 3.27× |
| 10,000 | 0.103 | 97.23M | 0.097 | 103.27M | 0.114 | 1.11× | 1.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.018 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.309 | 0.252 | 3.96M | 49.843 | 197.44× | 123.66× |
| 1,500 | 10 | 1.185 | 1.077 | 9.29M | 43.149 | 40.08× | 30.52× |
| 1,500 | 100 | 3.796 | 3.109 | 32.16M | 44.717 | 14.38× | 10.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.07M | 13.45M | 1.00× | 1.31M | 953.20K | 1.00× | 9.28M |
| 2 | 20.32M | 21.98M | 1.63× | 1.28M | 1.43M | 1.50× | 9.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
