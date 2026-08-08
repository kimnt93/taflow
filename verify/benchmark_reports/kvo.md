# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.91M | 0.012 | 86.04M | nan | — | — |
| 10,000 | 0.104 | 96.57M | 0.098 | 101.98M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.543 | 0.325 | 3.08M | nan | — | — |
| 1,500 | 10 | 1.941 | 1.260 | 7.94M | nan | — | — |
| 1,500 | 100 | 4.076 | 3.301 | 30.29M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
