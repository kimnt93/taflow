# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 216.38M | 0.003 | 325.44M | 0.029 | 6.20× | 9.32× |
| 10,000 | 0.010 | 985.74M | 0.007 | 1.54G | 0.040 | 3.91× | 6.09× |
| 100,000 | 0.062 | 1.61G | 0.039 | 2.59G | 0.066 | 1.06× | 1.71× |
| 1,000,000 | 1.089 | 918.59M | 0.713 | 1.40G | 0.775 | 0.71× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.112 | 0.66× |
| 1 | 5 | 0.354 | 0.551 | 1.56× |
| 1 | 10 | 0.490 | 0.953 | 1.94× |
| 10 | 1 | 0.049 | 0.091 | 1.86× |
| 10 | 5 | 0.225 | 0.418 | 1.86× |
| 10 | 10 | 0.477 | 0.942 | 1.97× |
| 100 | 1 | 0.055 | 0.105 | 1.90× |
| 100 | 5 | 0.242 | 0.477 | 1.98× |
| 100 | 10 | 0.497 | 0.926 | 1.86× |
| 1,000 | 1 | 0.054 | 0.090 | 1.67× |
| 1,000 | 5 | 0.228 | 0.446 | 1.96× |
| 1,000 | 10 | 0.466 | 0.891 | 1.91× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.186 | 5.37M | 66.387 | 356.22× | 146.20× |
| 100,000 | 10 | 1.496 | 0.774 | 12.91M | 66.516 | 85.89× | 34.84× |
| 100,000 | 1,000 | 3.821 | 2.125 | 470.68M | 68.014 | 32.01× | 13.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 524.26M | 890.82M | 1.00× | 2.46M | 3.61M | 1.00× | 577.67M |
| 5 | 813.05M | 1.69G | 1.90× | 2.27M | 3.00M | 0.83× | 565.38M |
| 10 | 702.15M | 1.36G | 1.52× | 2.05M | 2.72M | 0.75× | 580.26M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
