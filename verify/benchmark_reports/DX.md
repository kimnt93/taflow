# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.55M | 0.011 | 87.07M | 0.040 | 0.73× | 3.44× |
| 10,000 | 0.576 | 17.36M | 0.109 | 91.78M | 0.123 | 0.21× | 1.13× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.083 ms**; native kernel **0.018 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.386 | 0.909 | 1.10M | 44.076 | 48.46× | 34.18× |
| 1,500 | 10 | 1.885 | 1.116 | 8.96M | 44.027 | 39.46× | 28.55× |
| 1,500 | 100 | 8.094 | 3.512 | 28.47M | 50.122 | 14.27× | 10.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
