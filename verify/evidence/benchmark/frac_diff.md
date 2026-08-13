# FracDiff benchmark (`fixed-width fractional differencing` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.147 | 871.55K | 1.176 | 849.99K | 0.281 | 0.24× | 0.24× |
| 10,000 | 137.487 | 72.73K | 136.751 | 73.13K | 7.592 | 0.06× | 0.06× |
| 100,000 | 1492.568 | 67.00K | 1488.588 | 67.18K | 82.038 | 0.05× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.242 | 0.369 | 1.53× |
| 1 | 5 | 0.589 | 1.464 | 2.49× |
| 1 | 10 | 0.781 | 2.776 | 3.55× |
| 10 | 1 | 0.087 | 0.284 | 3.28× |
| 10 | 5 | 0.378 | 1.342 | 3.55× |
| 10 | 10 | 0.768 | 2.710 | 3.53× |
| 100 | 1 | 0.089 | 0.267 | 3.01× |
| 100 | 5 | 0.388 | 1.336 | 3.44× |
| 100 | 10 | 0.758 | 2.705 | 3.57× |
| 1,000 | 1 | 1.298 | 0.390 | 0.30× |
| 1,000 | 5 | 2.234 | 1.876 | 0.84× |
| 1,000 | 10 | 2.759 | 3.788 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
