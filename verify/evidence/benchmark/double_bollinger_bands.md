# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.546 | 1.83M | 0.531 | 1.88M | 0.615 | 1.13× | 1.16× |
| 10,000 | 5.321 | 1.88M | 5.405 | 1.85M | 4.094 | 0.77× | 0.76× |
| 100,000 | 56.138 | 1.78M | 53.743 | 1.86M | 48.821 | 0.87× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.296 | 2.83× |
| 1 | 5 | 0.387 | 1.352 | 3.50× |
| 1 | 10 | 0.621 | 2.835 | 4.56× |
| 10 | 1 | 0.075 | 0.253 | 3.36× |
| 10 | 5 | 0.306 | 1.361 | 4.45× |
| 10 | 10 | 0.649 | 2.815 | 4.34× |
| 100 | 1 | 0.128 | 0.292 | 2.28× |
| 100 | 5 | 0.307 | 1.560 | 5.08× |
| 100 | 10 | 0.673 | 3.194 | 4.74× |
| 1,000 | 1 | 0.636 | 0.810 | 1.27× |
| 1,000 | 5 | 1.095 | 3.935 | 3.59× |
| 1,000 | 10 | 1.461 | 7.838 | 5.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
