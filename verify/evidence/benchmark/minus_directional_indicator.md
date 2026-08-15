# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.09M | 0.007 | 148.48M | 0.041 | 4.75× | 6.08× |
| 10,000 | 0.065 | 153.32M | 0.061 | 163.15M | 0.111 | 1.69× | 1.80× |
| 100,000 | 0.657 | 152.23M | 0.606 | 164.98M | 0.706 | 1.07× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.129 | 1.47× |
| 1 | 5 | 0.247 | 0.499 | 2.02× |
| 1 | 10 | 0.388 | 0.978 | 2.52× |
| 10 | 1 | 0.041 | 0.091 | 2.24× |
| 10 | 5 | 0.191 | 0.429 | 2.25× |
| 10 | 10 | 0.404 | 0.978 | 2.42× |
| 100 | 1 | 0.047 | 0.101 | 2.16× |
| 100 | 5 | 0.201 | 0.453 | 2.25× |
| 100 | 10 | 0.399 | 1.005 | 2.52× |
| 1,000 | 1 | 0.052 | 0.109 | 2.10× |
| 1,000 | 5 | 0.202 | 0.492 | 2.44× |
| 1,000 | 10 | 0.414 | 1.028 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
