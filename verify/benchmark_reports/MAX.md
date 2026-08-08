# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.063 | 15.81M | 0.018 | 54.99M | 0.038 | 0.60× | 2.09× |
| 10,000 | 0.678 | 14.74M | 0.248 | 40.29M | 0.081 | 0.12× | 0.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.094 ms**; native kernel **0.030 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.339 | 0.210 | 4.76M | 36.633 | 174.44× | 153.29× |
| 1,500 | 10 | 2.080 | 0.959 | 10.42M | 36.651 | 38.20× | 32.41× |
| 1,500 | 100 | 10.043 | 4.424 | 22.60M | 38.943 | 8.80× | 7.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
