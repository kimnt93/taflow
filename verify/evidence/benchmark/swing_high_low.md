# SwingHighLow benchmark (`causal confirmed swing pivots` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.32M | 0.039 | 25.49M | 3.872 | 86.42× | 98.69× |
| 10,000 | 0.397 | 25.20M | 0.389 | 25.74M | 39.564 | 99.70× | 101.83× |
| 100,000 | 3.957 | 25.27M | 3.753 | 26.65M | 394.751 | 99.77× | 105.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.145 | 1.43× |
| 1 | 5 | 0.307 | 0.465 | 1.52× |
| 1 | 10 | 0.474 | 0.991 | 2.09× |
| 10 | 1 | 0.060 | 0.089 | 1.48× |
| 10 | 5 | 0.231 | 0.435 | 1.89× |
| 10 | 10 | 0.484 | 0.892 | 1.84× |
| 100 | 1 | 0.056 | 0.521 | 9.32× |
| 100 | 5 | 0.348 | 2.201 | 6.33× |
| 100 | 10 | 0.535 | 4.438 | 8.30× |
| 1,000 | 1 | 0.102 | 4.066 | 39.83× |
| 1,000 | 5 | 0.264 | 20.834 | 79.04× |
| 1,000 | 10 | 0.631 | 41.568 | 65.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
