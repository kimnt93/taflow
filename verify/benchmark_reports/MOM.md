# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.56M | 0.003 | 291.78M | 0.030 | 0.65× | 8.74× |
| 10,000 | 0.462 | 21.66M | 0.024 | 409.82M | 0.033 | 0.07× | 1.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.005 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.320 | 0.177 | 5.64M | 29.994 | 169.08× | 170.36× |
| 1,500 | 10 | 1.763 | 0.700 | 14.28M | 31.772 | 45.37× | 40.81× |
| 1,500 | 100 | 8.625 | 2.215 | 45.15M | 34.255 | 15.46× | 13.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
