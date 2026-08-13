# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.428 | 2.34M | 0.412 | 2.43M | 0.663 | 1.55× | 1.61× |
| 10,000 | 4.005 | 2.50M | 4.106 | 2.44M | 4.229 | 1.06× | 1.03× |
| 100,000 | 42.597 | 2.35M | 40.937 | 2.44M | 48.633 | 1.14× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.341 | 3.13× |
| 1 | 5 | 0.412 | 1.477 | 3.58× |
| 1 | 10 | 0.697 | 2.631 | 3.77× |
| 10 | 1 | 0.076 | 0.256 | 3.35× |
| 10 | 5 | 0.323 | 1.407 | 4.35× |
| 10 | 10 | 0.674 | 2.771 | 4.11× |
| 100 | 1 | 0.116 | 0.305 | 2.63× |
| 100 | 5 | 0.321 | 1.742 | 5.43× |
| 100 | 10 | 0.741 | 3.302 | 4.45× |
| 1,000 | 1 | 0.526 | 0.967 | 1.84× |
| 1,000 | 5 | 1.001 | 3.900 | 3.90× |
| 1,000 | 10 | 1.274 | 8.604 | 6.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
