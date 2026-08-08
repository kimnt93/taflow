# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.210 | 4.75M | 0.023 | 44.12M | 0.052 | 0.25× | 2.30× |
| 10,000 | 2.220 | 4.50M | 0.214 | 46.70M | 0.100 | 0.04× | 0.46× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.325 ms**; native kernel **0.033 ms**; TA-Lib 0.055 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.321 | 0.220 | 4.54M | 55.527 | 252.08× | 210.47× |
| 1,500 | 10 | 5.686 | 1.118 | 8.94M | 54.443 | 48.68× | 46.50× |
| 1,500 | 100 | 15.527 | 4.500 | 22.22M | 56.718 | 12.60× | 10.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
