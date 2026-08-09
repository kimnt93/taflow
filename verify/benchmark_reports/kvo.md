# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.74M | 0.011 | 87.30M | nan | — | — |
| 10,000 | 0.109 | 91.52M | 0.101 | 99.15M | nan | — | — |
| 100,000 | 1.042 | 95.94M | 0.960 | 104.13M | nan | — | — |
| 1,000,000 | 15.852 | 63.08M | 14.983 | 66.74M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.059 ms**; native kernel **1.003 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.425 | 0.285 | 3.51M | nan | — | — |
| 100,000 | 10 | 1.766 | 1.104 | 9.06M | nan | — | — |
| 100,000 | 1,000 | 13.989 | 11.551 | 86.57M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 81.31M | 86.21M | 1.00× | 1.68M | 2.05M | 1.00× | — |
| 2 | 64.54M | 86.78M | 1.01× | 1.79M | 2.21M | 1.08× | — |
| 4 | 76.39M | 85.96M | 1.00× | 1.80M | 2.22M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
