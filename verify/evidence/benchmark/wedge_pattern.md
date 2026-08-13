# WedgePattern benchmark (`Wedge` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.57M | 0.048 | 20.83M | 0.218 | 3.83× | 4.54× |
| 10,000 | 0.379 | 26.39M | 0.376 | 26.60M | 1.343 | 3.54× | 3.57× |
| 100,000 | 3.637 | 27.50M | 3.908 | 25.59M | 12.756 | 3.51× | 3.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.206 | 1.88× |
| 1 | 5 | 0.392 | 0.826 | 2.11× |
| 1 | 10 | 0.643 | 1.630 | 2.54× |
| 10 | 1 | 0.084 | 0.163 | 1.93× |
| 10 | 5 | 0.325 | 1.109 | 3.42× |
| 10 | 10 | 0.696 | 1.747 | 2.51× |
| 100 | 1 | 0.096 | 0.192 | 2.00× |
| 100 | 5 | 0.335 | 1.170 | 3.50× |
| 100 | 10 | 0.687 | 1.815 | 2.64× |
| 1,000 | 1 | 0.110 | 0.298 | 2.71× |
| 1,000 | 5 | 0.316 | 1.744 | 5.52× |
| 1,000 | 10 | 0.716 | 3.004 | 4.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
