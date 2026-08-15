# ChaikinMoneyFlow benchmark (`ChaikinMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.68M | 0.008 | 123.67M | 0.286 | 27.36× | 35.37× |
| 10,000 | 0.065 | 155.03M | 0.075 | 132.92M | 1.449 | 22.47× | 19.27× |
| 100,000 | 0.647 | 154.63M | 0.627 | 159.53M | 12.888 | 19.93× | 20.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.244 | 3.34× |
| 1 | 5 | 0.330 | 1.541 | 4.67× |
| 1 | 10 | 0.413 | 2.619 | 6.34× |
| 10 | 1 | 0.048 | 0.230 | 4.76× |
| 10 | 5 | 0.246 | 1.135 | 4.60× |
| 10 | 10 | 0.399 | 2.337 | 5.86× |
| 100 | 1 | 0.044 | 0.224 | 5.07× |
| 100 | 5 | 0.196 | 1.382 | 7.04× |
| 100 | 10 | 0.403 | 2.534 | 6.28× |
| 1,000 | 1 | 0.049 | 0.362 | 7.35× |
| 1,000 | 5 | 0.203 | 2.065 | 10.17× |
| 1,000 | 10 | 0.438 | 3.698 | 8.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
