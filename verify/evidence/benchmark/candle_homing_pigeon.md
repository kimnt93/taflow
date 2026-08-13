# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.098 | 10.23M | 0.086 | 11.63M | 0.030 | 0.31× | 0.35× |
| 10,000 | 0.781 | 12.80M | 0.861 | 11.61M | 0.094 | 0.12× | 0.11× |
| 100,000 | 7.875 | 12.70M | 7.367 | 13.57M | 0.734 | 0.09× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.151 | 1.38× |
| 1 | 5 | 0.456 | 0.487 | 1.07× |
| 1 | 10 | 0.633 | 0.910 | 1.44× |
| 10 | 1 | 0.065 | 0.090 | 1.37× |
| 10 | 5 | 0.323 | 0.438 | 1.36× |
| 10 | 10 | 0.646 | 0.910 | 1.41× |
| 100 | 1 | 0.082 | 0.083 | 1.01× |
| 100 | 5 | 0.295 | 0.428 | 1.45× |
| 100 | 10 | 0.682 | 0.921 | 1.35× |
| 1,000 | 1 | 0.152 | 0.098 | 0.64× |
| 1,000 | 5 | 0.333 | 0.471 | 1.41× |
| 1,000 | 10 | 0.791 | 1.068 | 1.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
