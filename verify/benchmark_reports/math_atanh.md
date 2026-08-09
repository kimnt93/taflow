# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.57M | 0.014 | 71.88M | 0.015 | 1.03× | 1.10× |
| 10,000 | 0.128 | 78.39M | 0.142 | 70.58M | 0.140 | 1.10× | 0.99× |
| 100,000 | 1.188 | 84.21M | 1.181 | 84.70M | 1.363 | 1.15× | 1.15× |
| 1,000,000 | 12.858 | 77.77M | 11.895 | 84.07M | 13.785 | 1.07× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.073 | 0.66× |
| 1 | 5 | 0.246 | 0.183 | 0.74× |
| 1 | 10 | 0.434 | 0.380 | 0.88× |
| 10 | 1 | 0.049 | 0.050 | 1.02× |
| 10 | 5 | 0.228 | 0.185 | 0.81× |
| 10 | 10 | 0.442 | 0.389 | 0.88× |
| 100 | 1 | 0.046 | 0.042 | 0.90× |
| 100 | 5 | 0.238 | 0.195 | 0.82× |
| 100 | 10 | 0.474 | 0.410 | 0.86× |
| 1,000 | 1 | 0.062 | 0.059 | 0.95× |
| 1,000 | 5 | 0.257 | 0.223 | 0.87× |
| 1,000 | 10 | 0.515 | 0.468 | 0.91× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.334 | 0.176 | 5.68M | nan | — | — |
| 100,000 | 10 | 1.320 | 0.608 | 16.44M | nan | — | — |
| 100,000 | 1,000 | 14.018 | 12.634 | 79.15M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.72M | 73.35M | 1.00× | 2.17M | 3.37M | 1.00× | — |
| 5 | 272.16M | 283.52M | 3.87× | 2.16M | 2.62M | 0.78× | — |
| 10 | 321.88M | 456.21M | 6.22× | 1.70M | 2.27M | 0.67× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
