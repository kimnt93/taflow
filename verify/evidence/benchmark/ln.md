# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.94M | 0.007 | 148.71M | 0.033 | 4.49× | 4.95× |
| 10,000 | 0.049 | 204.67M | 0.046 | 216.00M | 0.070 | 1.43× | 1.51× |
| 100,000 | 0.453 | 220.61M | 0.445 | 224.69M | 0.451 | 1.00× | 1.01× |
| 1,000,000 | 5.280 | 189.38M | 4.706 | 212.50M | 4.271 | 0.81× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.111 | 1.33× |
| 1 | 5 | 0.247 | 0.445 | 1.80× |
| 1 | 10 | 0.477 | 1.020 | 2.14× |
| 10 | 1 | 0.058 | 0.098 | 1.70× |
| 10 | 5 | 0.234 | 0.440 | 1.88× |
| 10 | 10 | 0.451 | 0.905 | 2.00× |
| 100 | 1 | 0.054 | 0.098 | 1.83× |
| 100 | 5 | 0.249 | 0.484 | 1.94× |
| 100 | 10 | 0.487 | 0.883 | 1.81× |
| 1,000 | 1 | 0.050 | 0.087 | 1.74× |
| 1,000 | 5 | 0.225 | 0.465 | 2.06× |
| 1,000 | 10 | 0.513 | 0.939 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
