# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.43M | 0.003 | 286.89M | 0.028 | 0.64× | 8.12× |
| 10,000 | 0.426 | 23.50M | 0.028 | 361.35M | 0.042 | 0.10× | 1.53× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.006 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.349 | 0.184 | 5.44M | 29.287 | 159.28× | 154.76× |
| 1,500 | 10 | 1.656 | 0.625 | 16.01M | 29.213 | 46.76× | 42.53× |
| 1,500 | 100 | 61.728 | 23.657 | 4.23M | 30.284 | 1.28× | 1.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
