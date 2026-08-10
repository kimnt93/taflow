# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.85M | 0.014 | 72.19M | 0.040 | 3.02× | 2.91× |
| 10,000 | 0.097 | 103.00M | 0.104 | 95.95M | 0.097 | 1.00× | 0.93× |
| 100,000 | 0.889 | 112.46M | 1.020 | 98.02M | 0.689 | 0.78× | 0.68× |
| 1,000,000 | 9.222 | 108.44M | 10.643 | 93.96M | 6.671 | 0.72× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.115 | 0.91× |
| 1 | 5 | 0.250 | 0.476 | 1.90× |
| 1 | 10 | 0.471 | 0.936 | 1.99× |
| 10 | 1 | 0.051 | 0.089 | 1.76× |
| 10 | 5 | 0.224 | 0.431 | 1.92× |
| 10 | 10 | 0.462 | 0.919 | 1.99× |
| 100 | 1 | 0.048 | 0.094 | 1.97× |
| 100 | 5 | 0.225 | 0.435 | 1.94× |
| 100 | 10 | 0.495 | 0.962 | 1.94× |
| 1,000 | 1 | 0.064 | 0.097 | 1.51× |
| 1,000 | 5 | 0.231 | 0.479 | 2.07× |
| 1,000 | 10 | 0.494 | 1.055 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
