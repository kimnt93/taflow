# TrianglePattern benchmark (`Triangle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.61M | 0.050 | 20.02M | 0.223 | 3.70× | 4.46× |
| 10,000 | 0.429 | 23.31M | 0.402 | 24.90M | 1.354 | 3.16× | 3.37× |
| 100,000 | 3.897 | 25.66M | 3.838 | 26.06M | 12.714 | 3.26× | 3.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.198 | 1.60× |
| 1 | 5 | 0.356 | 0.841 | 2.36× |
| 1 | 10 | 0.638 | 1.645 | 2.58× |
| 10 | 1 | 0.070 | 0.170 | 2.41× |
| 10 | 5 | 0.306 | 1.136 | 3.71× |
| 10 | 10 | 0.644 | 1.663 | 2.58× |
| 100 | 1 | 0.075 | 0.179 | 2.39× |
| 100 | 5 | 0.325 | 1.138 | 3.50× |
| 100 | 10 | 0.683 | 1.832 | 2.68× |
| 1,000 | 1 | 0.115 | 0.301 | 2.60× |
| 1,000 | 5 | 0.322 | 1.740 | 5.40× |
| 1,000 | 10 | 0.712 | 3.007 | 4.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
