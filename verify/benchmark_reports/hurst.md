# Hurst benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.120 | 8.30M | 0.125 | 8.03M | nan | — | — |
| 10,000 | 1.250 | 8.00M | 1.286 | 7.77M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.186 ms**; native kernel **0.182 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.412 | 0.367 | 2.72M | nan | — | — |
| 1,500 | 10 | 2.188 | 1.737 | 5.76M | nan | — | — |
| 1,500 | 100 | 13.869 | 13.759 | 7.27M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
