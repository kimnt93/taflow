# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.41M | 0.007 | 144.39M | 0.034 | 4.15× | 4.97× |
| 10,000 | 0.055 | 182.71M | 0.054 | 186.63M | 0.072 | 1.32× | 1.35× |
| 100,000 | 0.513 | 195.03M | 0.479 | 208.84M | 0.433 | 0.84× | 0.90× |
| 1,000,000 | 5.676 | 176.17M | 5.117 | 195.41M | 4.003 | 0.71× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.119 | 0.90× |
| 1 | 5 | 0.326 | 0.484 | 1.48× |
| 1 | 10 | 0.477 | 0.889 | 1.86× |
| 10 | 1 | 0.048 | 0.088 | 1.83× |
| 10 | 5 | 0.225 | 0.407 | 1.81× |
| 10 | 10 | 0.471 | 0.860 | 1.83× |
| 100 | 1 | 0.049 | 0.084 | 1.71× |
| 100 | 5 | 0.226 | 0.415 | 1.84× |
| 100 | 10 | 0.483 | 0.916 | 1.90× |
| 1,000 | 1 | 0.056 | 0.096 | 1.70× |
| 1,000 | 5 | 0.262 | 0.452 | 1.72× |
| 1,000 | 10 | 0.502 | 0.947 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
