# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.42M | 0.014 | 69.13M | 0.044 | 2.96× | 3.03× |
| 10,000 | 0.163 | 61.26M | 0.155 | 64.69M | 0.135 | 0.83× | 0.88× |
| 100,000 | 1.552 | 64.43M | 1.514 | 66.06M | 0.976 | 0.63× | 0.64× |
| 1,000,000 | 17.155 | 58.29M | 16.109 | 62.08M | 10.290 | 0.60× | 0.64× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.601 ms**; native kernel **1.520 ms**; TA-Lib 0.961 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.393 | 0.321 | 3.11M | 988.414 | 3078.21× | 117.59× |
| 100,000 | 10 | 1.964 | 2.512 | 3.98M | 1020.734 | 406.30× | 15.02× |
| 100,000 | 1,000 | 96.981 | 82.794 | 12.08M | 1037.449 | 12.53× | 0.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 53.23M | 57.56M | 1.00× | 1.64M | 1.98M | 1.00× | 73.38M |
| 2 | 94.84M | 107.51M | 1.87× | 1.65M | 1.87M | 0.94× | 81.30M |
| 4 | 147.46M | 210.44M | 3.66× | 1.72M | 1.78M | 0.90× | 80.52M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
