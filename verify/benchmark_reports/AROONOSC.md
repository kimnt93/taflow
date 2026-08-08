# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.68M | 0.041 | 24.20M | 0.046 | 0.54× | 1.12× |
| 10,000 | 0.935 | 10.69M | 0.516 | 19.38M | 0.137 | 0.15× | 0.27× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.135 ms**; native kernel **0.061 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.257 | 3.89M | 41.733 | 162.25× | 122.35× |
| 1,500 | 10 | 6.556 | 1.327 | 7.54M | 42.408 | 31.97× | 21.72× |
| 1,500 | 100 | 11.650 | 6.711 | 14.90M | 45.724 | 6.81× | 4.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
