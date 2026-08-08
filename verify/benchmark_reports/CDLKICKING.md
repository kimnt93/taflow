# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.10M | 0.043 | 23.14M | 0.040 | 0.88× | 0.92× |
| 10,000 | 0.466 | 21.45M | 0.501 | 19.98M | 0.193 | 0.41× | 0.38× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.065 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.424 | 0.329 | 3.04M | 44.805 | 136.02× | 85.56× |
| 1,500 | 10 | 3.097 | 3.704 | 2.70M | 46.067 | 12.44× | 7.53× |
| 1,500 | 100 | 15.033 | 7.156 | 13.97M | 48.365 | 6.76× | 4.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
