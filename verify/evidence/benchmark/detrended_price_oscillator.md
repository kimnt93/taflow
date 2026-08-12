# DetrendedPriceOscillator benchmark (`dpo` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.99M | 0.008 | 130.99M | 0.345 | 41.10× | 45.24× |
| 10,000 | 0.056 | 179.78M | 0.052 | 192.14M | 0.415 | 7.46× | 7.97× |
| 100,000 | 0.508 | 196.90M | 0.477 | 209.70M | 1.428 | 2.81× | 2.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.183 | 1.26× |
| 1 | 5 | 0.376 | 0.760 | 2.02× |
| 1 | 10 | 0.472 | 1.587 | 3.36× |
| 10 | 1 | 0.051 | 0.169 | 3.33× |
| 10 | 5 | 0.245 | 0.769 | 3.14× |
| 10 | 10 | 0.479 | 1.719 | 3.59× |
| 100 | 1 | 0.052 | 0.435 | 8.32× |
| 100 | 5 | 0.260 | 2.051 | 7.89× |
| 100 | 10 | 0.514 | 4.000 | 7.78× |
| 1,000 | 1 | 0.061 | 0.397 | 6.49× |
| 1,000 | 5 | 0.236 | 2.036 | 8.62× |
| 1,000 | 10 | 0.510 | 4.216 | 8.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
