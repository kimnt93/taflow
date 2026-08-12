# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.66M | 0.005 | 188.03M | 0.028 | 4.09× | 5.25× |
| 10,000 | 0.021 | 470.37M | 0.018 | 556.06M | 0.035 | 1.65× | 1.95× |
| 100,000 | 0.172 | 581.24M | 0.132 | 760.40M | 0.082 | 0.48× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.098 | 1.26× |
| 1 | 5 | 0.272 | 0.481 | 1.77× |
| 1 | 10 | 0.509 | 0.918 | 1.80× |
| 10 | 1 | 0.053 | 0.092 | 1.74× |
| 10 | 5 | 0.218 | 0.432 | 1.98× |
| 10 | 10 | 0.485 | 0.874 | 1.80× |
| 100 | 1 | 0.050 | 0.090 | 1.81× |
| 100 | 5 | 0.238 | 0.442 | 1.86× |
| 100 | 10 | 0.495 | 0.899 | 1.82× |
| 1,000 | 1 | 0.049 | 0.087 | 1.77× |
| 1,000 | 5 | 0.231 | 0.427 | 1.85× |
| 1,000 | 10 | 0.501 | 0.875 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
