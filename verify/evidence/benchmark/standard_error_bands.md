# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 15.00M | 0.070 | 14.32M | 0.610 | 9.16× | 8.74× |
| 10,000 | 0.686 | 14.57M | 0.690 | 14.49M | 4.208 | 6.13× | 6.10× |
| 100,000 | 7.194 | 13.90M | 7.007 | 14.27M | 45.684 | 6.35× | 6.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.305 | 5.46× |
| 1 | 5 | 0.265 | 8.014 | 30.29× |
| 1 | 10 | 0.476 | 2.517 | 5.29× |
| 10 | 1 | 0.059 | 0.273 | 4.59× |
| 10 | 5 | 0.212 | 1.414 | 6.67× |
| 10 | 10 | 0.389 | 2.664 | 6.86× |
| 100 | 1 | 0.056 | 0.287 | 5.14× |
| 100 | 5 | 0.194 | 1.568 | 8.10× |
| 100 | 10 | 0.438 | 3.187 | 7.27× |
| 1,000 | 1 | 0.119 | 0.795 | 6.69× |
| 1,000 | 5 | 0.246 | 3.837 | 15.61× |
| 1,000 | 10 | 0.506 | 7.765 | 15.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
