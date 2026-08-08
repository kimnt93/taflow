# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.94M | 0.038 | 26.33M | 0.051 | 1.28× | 1.35× |
| 10,000 | 0.361 | 27.69M | 0.363 | 27.56M | 0.199 | 0.55× | 0.55× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.058 ms**; native kernel **0.056 ms**; TA-Lib 0.061 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.368 | 0.275 | 3.64M | 61.148 | 222.35× | 129.06× |
| 1,500 | 10 | 2.628 | 1.325 | 7.55M | 59.639 | 45.01× | 26.64× |
| 1,500 | 100 | 23.158 | 6.630 | 15.08M | 66.197 | 9.98× | 5.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
