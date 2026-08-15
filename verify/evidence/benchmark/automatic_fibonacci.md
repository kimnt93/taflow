# AutomaticFibonacci benchmark (`AutoFib` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.47M | 0.022 | 45.40M | 0.715 | 26.80× | 32.47× |
| 10,000 | 0.250 | 39.94M | 0.223 | 44.93M | 5.987 | 23.91× | 26.90× |
| 100,000 | 2.653 | 37.70M | 2.392 | 41.81M | 67.466 | 25.43× | 28.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.228 | 3.45× |
| 1 | 5 | 0.223 | 0.821 | 3.68× |
| 1 | 10 | 0.405 | 1.869 | 4.62× |
| 10 | 1 | 0.050 | 0.177 | 3.50× |
| 10 | 5 | 0.203 | 0.845 | 4.16× |
| 10 | 10 | 0.421 | 1.927 | 4.58× |
| 100 | 1 | 0.049 | 0.232 | 4.69× |
| 100 | 5 | 0.231 | 1.150 | 4.97× |
| 100 | 10 | 0.463 | 3.532 | 7.62× |
| 1,000 | 1 | 0.087 | 1.088 | 12.50× |
| 1,000 | 5 | 0.314 | 5.712 | 18.17× |
| 1,000 | 10 | 0.568 | 15.564 | 27.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
