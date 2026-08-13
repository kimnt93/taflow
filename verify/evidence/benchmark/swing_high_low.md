# SwingHighLow benchmark (`causal confirmed swing pivots` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.234 | 4.26M | 0.221 | 4.53M | 3.700 | 15.78× | 16.75× |
| 10,000 | 2.097 | 4.77M | 2.050 | 4.88M | 35.724 | 17.04× | 17.42× |
| 100,000 | 20.360 | 4.91M | 20.077 | 4.98M | 361.452 | 17.75× | 18.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.132 | 1.14× |
| 1 | 5 | 0.380 | 0.432 | 1.14× |
| 1 | 10 | 0.657 | 0.868 | 1.32× |
| 10 | 1 | 0.072 | 0.085 | 1.18× |
| 10 | 5 | 0.329 | 0.441 | 1.34× |
| 10 | 10 | 0.696 | 0.894 | 1.28× |
| 100 | 1 | 0.092 | 0.452 | 4.89× |
| 100 | 5 | 0.309 | 2.185 | 7.07× |
| 100 | 10 | 0.668 | 4.381 | 6.56× |
| 1,000 | 1 | 0.290 | 3.926 | 13.56× |
| 1,000 | 5 | 0.584 | 19.768 | 33.84× |
| 1,000 | 10 | 1.074 | 81.201 | 75.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
