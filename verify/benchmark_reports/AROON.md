# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.150 | 6.68M | 0.041 | 24.49M | 0.042 | 0.28× | 1.03× |
| 10,000 | 1.286 | 7.78M | 0.469 | 21.31M | 0.156 | 0.12× | 0.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.190 ms**; native kernel **0.066 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.474 | 0.282 | 3.54M | 45.158 | 160.01× | 125.29× |
| 1,500 | 10 | 3.443 | 1.390 | 7.19M | 45.256 | 32.55× | 30.09× |
| 1,500 | 100 | 15.634 | 7.108 | 14.07M | 46.686 | 6.57× | 5.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
