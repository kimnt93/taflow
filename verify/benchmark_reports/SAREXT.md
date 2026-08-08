# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.50M | 0.011 | 91.23M | 0.052 | 1.01× | 4.72× |
| 10,000 | 0.505 | 19.80M | 0.111 | 90.12M | 0.093 | 0.18× | 0.83× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.076 ms**; native kernel **0.016 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.344 | 0.216 | 4.64M | 51.821 | 240.36× | 214.40× |
| 1,500 | 10 | 1.537 | 2.485 | 4.02M | 51.416 | 20.69× | 18.46× |
| 1,500 | 100 | 7.163 | 3.406 | 29.36M | 52.022 | 15.27× | 13.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
