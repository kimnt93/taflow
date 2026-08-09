# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.88M | 0.009 | 107.63M | 0.038 | 3.57× | 4.05× |
| 10,000 | 0.120 | 83.16M | 0.114 | 87.74M | 0.129 | 1.07× | 1.13× |
| 100,000 | 1.162 | 86.09M | 1.137 | 87.93M | 1.057 | 0.91× | 0.93× |
| 1,000,000 | 12.234 | 81.74M | 11.632 | 85.97M | 9.890 | 0.81× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.181 ms**; native kernel **1.164 ms**; TA-Lib 1.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.300 | 0.219 | 4.56M | 1010.152 | 4608.46× | 137.93× |
| 100,000 | 10 | 1.646 | 0.964 | 10.38M | 1007.847 | 1045.80× | 32.07× |
| 100,000 | 1,000 | 36.219 | 27.796 | 35.98M | 1016.457 | 36.57× | 1.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 74.19M | 76.85M | 1.00× | 2.28M | 2.69M | 1.00× | 82.50M |
| 2 | 135.67M | 147.19M | 1.92× | 2.47M | 2.90M | 1.08× | 83.01M |
| 4 | 223.91M | 278.27M | 3.62× | 2.28M | 2.63M | 0.98× | 81.87M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
