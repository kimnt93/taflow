# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.82M | 0.077 | 12.99M | 0.359 | 4.25× | 4.67× |
| 10,000 | 0.615 | 16.27M | 0.611 | 16.36M | 2.317 | 3.77× | 3.79× |
| 100,000 | 6.029 | 16.59M | 5.729 | 17.45M | 22.747 | 3.77× | 3.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.201 | 0.265 | 1.32× |
| 1 | 5 | 0.648 | 1.164 | 1.80× |
| 1 | 10 | 0.691 | 2.490 | 3.60× |
| 10 | 1 | 0.077 | 0.225 | 2.94× |
| 10 | 5 | 0.338 | 1.097 | 3.25× |
| 10 | 10 | 0.694 | 2.389 | 3.44× |
| 100 | 1 | 0.092 | 0.245 | 2.65× |
| 100 | 5 | 0.341 | 1.366 | 4.00× |
| 100 | 10 | 0.726 | 2.570 | 3.54× |
| 1,000 | 1 | 0.151 | 0.460 | 3.05× |
| 1,000 | 5 | 0.353 | 2.511 | 7.10× |
| 1,000 | 10 | 0.737 | 4.633 | 6.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
