# PivotPoints benchmark (`anchored classic pivot points` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.35M | 0.011 | 88.24M | 0.990 | 59.73× | 87.35× |
| 10,000 | 0.115 | 87.07M | 0.092 | 108.87M | 10.275 | 89.46× | 111.86× |
| 100,000 | 1.152 | 86.80M | 0.837 | 119.46M | 90.393 | 78.46× | 107.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.145 | 1.00× |
| 1 | 5 | 0.286 | 0.391 | 1.37× |
| 1 | 10 | 0.409 | 0.697 | 1.71× |
| 10 | 1 | 0.041 | 0.086 | 2.10× |
| 10 | 5 | 0.184 | 0.387 | 2.11× |
| 10 | 10 | 0.391 | 0.885 | 2.26× |
| 100 | 1 | 0.049 | 0.171 | 3.48× |
| 100 | 5 | 0.201 | 0.834 | 4.15× |
| 100 | 10 | 0.425 | 1.748 | 4.12× |
| 1,000 | 1 | 0.056 | 1.007 | 17.83× |
| 1,000 | 5 | 0.262 | 5.058 | 19.33× |
| 1,000 | 10 | 0.525 | 10.567 | 20.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
