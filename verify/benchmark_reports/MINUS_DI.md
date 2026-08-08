# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.38M | 0.011 | 92.81M | 0.041 | 3.45× | 3.84× |
| 10,000 | 0.102 | 98.47M | 0.097 | 102.98M | 0.101 | 1.00× | 1.04× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.016 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.338 | 0.255 | 3.93M | 42.598 | 167.37× | 135.46× |
| 1,500 | 10 | 2.268 | 1.073 | 9.32M | 50.158 | 46.73× | 29.37× |
| 1,500 | 100 | 6.924 | 3.929 | 25.45M | 45.426 | 11.56× | 8.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
