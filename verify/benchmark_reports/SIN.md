# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.22M | 0.011 | 90.51M | 0.036 | 0.66× | 3.28× |
| 10,000 | 0.574 | 17.42M | 0.151 | 66.18M | 0.183 | 0.32× | 1.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.083 ms**; native kernel **0.017 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.389 | 0.194 | 5.17M | 46.017 | 237.75× | 131.56× |
| 1,500 | 10 | 4.125 | 0.793 | 12.61M | 45.551 | 57.43× | 33.69× |
| 1,500 | 100 | 13.161 | 3.659 | 27.33M | 45.431 | 12.42× | 7.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
