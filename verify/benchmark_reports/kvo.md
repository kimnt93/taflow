# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.26M | 0.010 | 97.04M | nan | — | — |
| 10,000 | 0.093 | 107.63M | 0.088 | 114.17M | nan | — | — |
| 100,000 | 0.887 | 112.68M | 0.850 | 117.66M | nan | — | — |
| 1,000,000 | 10.061 | 99.40M | 9.121 | 109.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.891 ms**; native kernel **0.848 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.534 | 0.266 | 3.75M | nan | — | — |
| 100,000 | 10 | 1.608 | 1.059 | 9.44M | nan | — | — |
| 100,000 | 1,000 | 11.391 | 10.526 | 95.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.96M | 93.43M | 1.00× | 1.75M | 2.15M | 1.00× | — |
| 2 | 83.49M | 95.18M | 1.02× | 1.80M | 2.28M | 1.06× | — |
| 4 | 82.07M | 90.67M | 0.97× | 1.78M | 2.34M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
