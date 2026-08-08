# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.52M | 0.005 | 188.18M | nan | — | — |
| 10,000 | 0.478 | 20.90M | 0.045 | 220.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.381 | 0.192 | 5.21M | nan | — | — |
| 1,500 | 10 | 1.745 | 0.710 | 14.08M | nan | — | — |
| 1,500 | 100 | 6.798 | 2.270 | 44.05M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
