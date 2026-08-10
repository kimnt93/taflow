# GapUp benchmark (`gap up relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.08M | 0.006 | 162.35M | 0.022 | 3.07× | 3.61× |
| 10,000 | 0.031 | 325.03M | 0.028 | 362.64M | 0.042 | 1.36× | 1.52× |
| 100,000 | 0.312 | 320.77M | 0.260 | 383.99M | 0.260 | 0.83× | 1.00× |
| 1,000,000 | 3.218 | 310.73M | 2.861 | 349.49M | 5.408 | 1.68× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.171 | 0.137 | 0.80× |
| 1 | 5 | 0.290 | 0.367 | 1.27× |
| 1 | 10 | 0.511 | 1.820 | 3.56× |
| 10 | 1 | 0.064 | 0.078 | 1.23× |
| 10 | 5 | 0.312 | 0.473 | 1.51× |
| 10 | 10 | 0.677 | 1.023 | 1.51× |
| 100 | 1 | 0.071 | 0.093 | 1.31× |
| 100 | 5 | 0.318 | 0.495 | 1.56× |
| 100 | 10 | 0.624 | 0.933 | 1.50× |
| 1,000 | 1 | 0.085 | 0.099 | 1.16× |
| 1,000 | 5 | 0.339 | 0.725 | 2.13× |
| 1,000 | 10 | 0.663 | 1.266 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
