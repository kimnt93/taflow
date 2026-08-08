# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 31.97M | 0.029 | 33.95M | 0.040 | 1.27× | 1.35× |
| 10,000 | 0.386 | 25.91M | 0.381 | 26.25M | 0.138 | 0.36× | 0.36× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.049 ms**; native kernel **0.046 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.037 | 0.333 | 3.00M | 45.888 | 137.80× | 85.81× |
| 1,500 | 10 | 2.970 | 1.489 | 6.72M | 43.708 | 29.35× | 19.36× |
| 1,500 | 100 | 11.761 | 6.711 | 14.90M | 46.567 | 6.94× | 4.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
