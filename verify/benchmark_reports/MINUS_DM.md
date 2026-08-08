# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.48M | 0.006 | 168.72M | 0.037 | 4.98× | 6.25× |
| 10,000 | 0.055 | 181.97M | 0.053 | 188.08M | 0.084 | 1.53× | 1.58× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.292 | 0.216 | 4.63M | 38.984 | 180.51× | 147.27× |
| 1,500 | 10 | 2.569 | 1.263 | 7.92M | 37.902 | 30.01× | 32.15× |
| 1,500 | 100 | 8.985 | 5.144 | 19.44M | 43.362 | 8.43× | 6.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
