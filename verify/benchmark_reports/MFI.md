# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.44M | 0.012 | 82.61M | 0.038 | 2.59× | 3.13× |
| 10,000 | 0.158 | 63.17M | 0.156 | 64.22M | 0.116 | 0.73× | 0.75× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.020 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.464 | 0.305 | 3.28M | 38.453 | 126.08× | 112.07× |
| 1,500 | 10 | 2.802 | 1.360 | 7.35M | 38.266 | 28.13× | 23.68× |
| 1,500 | 100 | 7.543 | 4.600 | 21.74M | 38.008 | 8.26× | 7.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
