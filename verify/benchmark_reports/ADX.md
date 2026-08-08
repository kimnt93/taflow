# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.16M | 0.015 | 67.20M | 0.043 | 0.74× | 2.89× |
| 10,000 | 0.571 | 17.52M | 0.138 | 72.48M | 0.126 | 0.22× | 0.92× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.085 ms**; native kernel **0.022 ms**; TA-Lib 0.047 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.397 | 0.258 | 3.88M | 46.233 | 179.40× | 123.98× |
| 1,500 | 10 | 1.989 | 1.160 | 8.62M | 45.703 | 39.40× | 27.75× |
| 1,500 | 100 | 7.853 | 3.808 | 26.26M | 44.359 | 11.65× | 8.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
