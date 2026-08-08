# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.20M | 0.027 | 36.61M | nan | — | — |
| 10,000 | 0.270 | 36.97M | 0.274 | 36.46M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.043 ms**; native kernel **0.041 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.331 | 0.239 | 4.19M | nan | — | — |
| 1,500 | 10 | 1.771 | 0.998 | 10.02M | nan | — | — |
| 1,500 | 100 | 5.207 | 4.296 | 23.28M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
