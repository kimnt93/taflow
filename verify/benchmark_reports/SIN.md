# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.18M | 0.014 | 70.38M | 0.036 | 2.58× | 2.55× |
| 10,000 | 0.155 | 64.54M | 0.154 | 65.06M | 0.173 | 1.12× | 1.13× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.019 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.395 | 0.296 | 3.37M | 42.808 | 144.41× | 83.93× |
| 1,500 | 10 | 1.260 | 0.719 | 13.91M | 42.896 | 59.67× | 35.64× |
| 1,500 | 100 | 4.451 | 3.105 | 32.20M | 45.063 | 14.51× | 8.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.74M | 8.42M | 1.00× | 774.02K | 1.16M | 1.00× | 8.69M |
| 2 | 17.29M | 20.84M | 2.47× | 1.35M | 1.65M | 1.42× | 9.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
