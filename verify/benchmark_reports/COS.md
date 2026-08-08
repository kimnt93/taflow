# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.60M | 0.011 | 87.96M | 0.042 | 0.78× | 3.71× |
| 10,000 | 0.565 | 17.70M | 0.152 | 65.96M | 0.175 | 0.31× | 1.16× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.084 ms**; native kernel **0.018 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.353 | 0.187 | 5.35M | 45.216 | 241.96× | 143.71× |
| 1,500 | 10 | 1.864 | 0.804 | 12.44M | 47.414 | 58.99× | 33.86× |
| 1,500 | 100 | 8.708 | 3.591 | 27.85M | 47.900 | 13.34× | 7.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
