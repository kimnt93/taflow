# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.09M | 0.005 | 205.00M | 0.036 | 5.40× | 7.37× |
| 10,000 | 0.074 | 134.53M | 0.071 | 139.96M | 0.141 | 1.90× | 1.97× |
| 100,000 | 0.951 | 105.19M | 0.929 | 107.59M | 1.141 | 1.20× | 1.23× |
| 1,000,000 | 9.718 | 102.90M | 9.828 | 101.75M | 11.172 | 1.15× | 1.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.943 ms**; native kernel **0.933 ms**; TA-Lib 1.161 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.337 | 0.266 | 3.76M | 1135.372 | 4267.10× | 102.81× |
| 100,000 | 10 | 2.741 | 1.326 | 7.54M | 1139.192 | 858.96× | 21.37× |
| 100,000 | 1,000 | 28.211 | 25.840 | 38.70M | 1161.892 | 44.97× | 1.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.11M | 94.96M | 1.00× | 2.10M | 2.89M | 1.00× | 77.34M |
| 2 | 172.44M | 174.28M | 1.84× | 2.50M | 2.68M | 0.93× | 77.65M |
| 4 | 319.88M | 331.99M | 3.50× | 2.30M | 2.48M | 0.86× | 77.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
