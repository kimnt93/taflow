# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.58M | 0.009 | 108.85M | 0.035 | 0.69× | 3.86× |
| 10,000 | 0.488 | 20.47M | 0.092 | 108.15M | 0.108 | 0.22× | 1.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.079 ms**; native kernel **0.014 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.350 | 0.190 | 5.26M | 39.038 | 205.25× | 142.74× |
| 1,500 | 10 | 3.251 | 1.249 | 8.01M | 41.474 | 33.21× | 20.79× |
| 1,500 | 100 | 8.026 | 3.426 | 29.19M | 41.067 | 11.99× | 7.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
