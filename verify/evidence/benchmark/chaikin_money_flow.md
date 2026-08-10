# ChaikinMoneyFlow benchmark (`ChaikinMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.89M | 0.012 | 85.05M | 0.283 | 18.07× | 24.05× |
| 10,000 | 0.069 | 144.18M | 0.071 | 141.40M | 1.433 | 20.67× | 20.27× |
| 100,000 | 0.611 | 163.59M | 0.576 | 173.47M | 12.699 | 20.77× | 22.03× |
| 1,000,000 | 6.579 | 151.99M | 5.920 | 168.92M | 128.029 | 19.46× | 21.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.651 | 6.39× |
| 1 | 5 | 0.364 | 1.184 | 3.25× |
| 1 | 10 | 0.536 | 2.643 | 4.93× |
| 10 | 1 | 0.065 | 0.224 | 3.43× |
| 10 | 5 | 0.244 | 1.110 | 4.55× |
| 10 | 10 | 0.547 | 2.428 | 4.44× |
| 100 | 1 | 0.053 | 0.238 | 4.45× |
| 100 | 5 | 0.266 | 1.478 | 5.56× |
| 100 | 10 | 0.561 | 2.638 | 4.70× |
| 1,000 | 1 | 0.081 | 0.356 | 4.42× |
| 1,000 | 5 | 0.332 | 2.356 | 7.10× |
| 1,000 | 10 | 0.551 | 3.969 | 7.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
