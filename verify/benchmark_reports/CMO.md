# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.99M | 0.006 | 167.02M | 0.037 | 5.34× | 6.15× |
| 10,000 | 0.057 | 174.49M | 0.053 | 189.95M | 0.086 | 1.50× | 1.64× |
| 100,000 | 0.547 | 182.82M | 0.514 | 194.71M | 0.573 | 1.05× | 1.11× |
| 1,000,000 | 5.518 | 181.24M | 5.212 | 191.87M | 5.551 | 1.01× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.534 ms**; native kernel **0.507 ms**; TA-Lib 0.575 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.151 | 6.60M | 570.051 | 3764.02× | 201.08× |
| 100,000 | 10 | 0.927 | 0.513 | 19.50M | 574.375 | 1119.84× | 60.49× |
| 100,000 | 1,000 | 8.432 | 6.550 | 152.68M | 600.008 | 91.61× | 5.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 143.55M | 160.73M | 1.00× | 3.09M | 3.94M | 1.00× | 150.46M |
| 2 | 232.20M | 295.19M | 1.84× | 3.11M | 4.09M | 1.04× | 139.54M |
| 4 | 392.09M | 488.61M | 3.04× | 2.87M | 3.01M | 0.76× | 130.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
