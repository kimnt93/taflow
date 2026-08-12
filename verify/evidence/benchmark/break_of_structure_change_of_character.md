# BreakOfStructureChangeOfCharacter benchmark (`causal BOS and CHOCH events` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.33M | 0.044 | 22.97M | 4.268 | 86.76× | 98.04× |
| 10,000 | 0.448 | 22.31M | 0.755 | 13.24M | 43.398 | 96.82× | 57.46× |
| 100,000 | 4.737 | 21.11M | 4.697 | 21.29M | 436.129 | 92.06× | 92.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.185 | 1.24× |
| 1 | 5 | 0.349 | 0.505 | 1.45× |
| 1 | 10 | 0.523 | 1.002 | 1.92× |
| 10 | 1 | 0.058 | 0.111 | 1.92× |
| 10 | 5 | 0.266 | 0.542 | 2.04× |
| 10 | 10 | 0.554 | 1.052 | 1.90× |
| 100 | 1 | 0.058 | 0.488 | 8.36× |
| 100 | 5 | 0.273 | 2.603 | 9.52× |
| 100 | 10 | 0.568 | 4.909 | 8.65× |
| 1,000 | 1 | 0.101 | 4.236 | 42.15× |
| 1,000 | 5 | 0.380 | 22.517 | 59.33× |
| 1,000 | 10 | 0.666 | 46.029 | 69.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
