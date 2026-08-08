# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.03M | 0.013 | 79.97M | 0.056 | 3.86× | 4.47× |
| 10,000 | 0.132 | 75.81M | 0.119 | 83.88M | 0.197 | 1.50× | 1.66× |
| 100,000 | 1.305 | 76.65M | 1.135 | 88.08M | 1.657 | 1.27× | 1.46× |
| 1,000,000 | 12.438 | 80.40M | 11.731 | 85.25M | 15.632 | 1.26× | 1.33× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.150 ms**; native kernel **1.140 ms**; TA-Lib 1.592 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.312 | 0.242 | 4.13M | 1567.604 | 6471.82× | 146.30× |
| 100,000 | 10 | 2.156 | 1.210 | 8.26M | 1521.435 | 1257.27× | 29.80× |
| 100,000 | 1,000 | 15.446 | 14.263 | 70.11M | 1583.475 | 111.02× | 3.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.60M | 59.71M | 1.00× | 2.06M | 2.46M | 1.00× | 57.85M |
| 2 | 119.23M | 134.53M | 2.25× | 2.23M | 2.63M | 1.07× | 50.18M |
| 4 | 241.87M | 228.98M | 3.83× | 2.28M | 2.40M | 0.98× | 58.12M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
