# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.30M | 0.008 | 122.84M | 0.034 | 0.69× | 4.17× |
| 10,000 | 0.477 | 20.98M | 0.078 | 127.87M | 0.095 | 0.20× | 1.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.075 ms**; native kernel **0.012 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.303 | 0.188 | 5.33M | 37.301 | 198.92× | 133.98× |
| 1,500 | 10 | 3.707 | 0.767 | 13.04M | 37.156 | 48.45× | 34.77× |
| 1,500 | 100 | 7.505 | 3.516 | 28.44M | 39.593 | 11.26× | 8.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
