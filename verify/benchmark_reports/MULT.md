# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.251 | 3.99M | 0.001 | 859.42M | 0.031 | 0.12× | 26.44× |
| 10,000 | 2.844 | 3.52M | 0.009 | 1.08G | 0.037 | 0.01× | 4.01× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.405 ms**; native kernel **0.001 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.775 | 0.219 | 4.57M | 30.250 | 138.17× | 134.18× |
| 1,500 | 10 | 4.631 | 0.860 | 11.63M | 28.776 | 33.45× | 33.77× |
| 1,500 | 100 | 27.369 | 2.503 | 39.95M | 33.962 | 13.57× | 11.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
