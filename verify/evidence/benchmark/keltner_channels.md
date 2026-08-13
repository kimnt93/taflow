# KeltnerChannels benchmark (`Keltner` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.50M | 0.093 | 10.74M | 0.611 | 5.81× | 6.56× |
| 10,000 | 0.822 | 12.16M | 0.834 | 11.99M | 4.288 | 5.21× | 5.14× |
| 100,000 | 8.298 | 12.05M | 7.749 | 12.90M | 45.712 | 5.51× | 5.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.335 | 2.98× |
| 1 | 5 | 0.504 | 1.621 | 3.22× |
| 1 | 10 | 0.714 | 3.205 | 4.49× |
| 10 | 1 | 0.080 | 0.291 | 3.64× |
| 10 | 5 | 0.322 | 1.575 | 4.90× |
| 10 | 10 | 0.715 | 3.205 | 4.48× |
| 100 | 1 | 0.093 | 0.330 | 3.54× |
| 100 | 5 | 0.350 | 1.758 | 5.02× |
| 100 | 10 | 0.680 | 3.718 | 5.47× |
| 1,000 | 1 | 0.167 | 0.797 | 4.78× |
| 1,000 | 5 | 0.372 | 3.931 | 10.57× |
| 1,000 | 10 | 0.749 | 7.907 | 10.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
