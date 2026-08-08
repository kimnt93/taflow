# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.19M | 0.003 | 289.88M | 0.029 | 0.62× | 8.49× |
| 10,000 | 0.427 | 23.43M | 0.027 | 366.94M | 0.045 | 0.10× | 1.64× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.005 ms**; TA-Lib 0.028 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.301 | 0.176 | 5.69M | 30.252 | 172.08× | 142.71× |
| 1,500 | 10 | 1.615 | 0.647 | 15.45M | 29.378 | 45.40× | 38.95× |
| 1,500 | 100 | 6.633 | 2.396 | 41.74M | 29.090 | 12.14× | 10.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
