# FibonacciFan benchmark (`FibFan` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.41M | 0.016 | 61.29M | 0.513 | 26.88× | 31.44× |
| 10,000 | 0.146 | 68.45M | 0.137 | 72.84M | 4.087 | 27.98× | 29.77× |
| 100,000 | 1.449 | 68.99M | 1.313 | 76.14M | 43.698 | 30.15× | 33.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.222 | 2.28× |
| 1 | 5 | 0.273 | 0.937 | 3.43× |
| 1 | 10 | 0.539 | 2.042 | 3.79× |
| 10 | 1 | 0.055 | 0.175 | 3.21× |
| 10 | 5 | 0.229 | 0.942 | 4.11× |
| 10 | 10 | 0.497 | 1.970 | 3.97× |
| 100 | 1 | 0.053 | 0.217 | 4.08× |
| 100 | 5 | 0.291 | 1.113 | 3.83× |
| 100 | 10 | 0.538 | 2.418 | 4.49× |
| 1,000 | 1 | 0.075 | 0.816 | 10.90× |
| 1,000 | 5 | 0.256 | 3.166 | 12.36× |
| 1,000 | 10 | 0.572 | 6.447 | 11.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
