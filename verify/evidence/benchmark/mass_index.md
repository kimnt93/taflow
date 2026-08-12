# MassIndex benchmark (`MassIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.86M | 0.013 | 75.23M | 0.250 | 17.45× | 18.79× |
| 10,000 | 0.100 | 100.15M | 0.093 | 107.03M | 0.855 | 8.56× | 9.15× |
| 100,000 | 0.878 | 113.90M | 0.836 | 119.68M | 7.425 | 8.46× | 8.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.341 | 3.14× |
| 1 | 5 | 0.256 | 1.529 | 5.97× |
| 1 | 10 | 0.520 | 3.145 | 6.04× |
| 10 | 1 | 0.056 | 0.278 | 4.93× |
| 10 | 5 | 0.230 | 1.688 | 7.33× |
| 10 | 10 | 0.523 | 2.849 | 5.45× |
| 100 | 1 | 0.057 | 0.270 | 4.72× |
| 100 | 5 | 0.267 | 1.636 | 6.13× |
| 100 | 10 | 0.521 | 3.199 | 6.14× |
| 1,000 | 1 | 0.066 | 0.325 | 4.95× |
| 1,000 | 5 | 0.242 | 2.001 | 8.28× |
| 1,000 | 10 | 0.552 | 3.496 | 6.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
