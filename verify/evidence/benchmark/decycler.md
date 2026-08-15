# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.07M | 0.005 | 188.43M | 0.144 | 22.22× | 27.17× |
| 10,000 | 0.046 | 217.32M | 0.043 | 234.92M | 0.467 | 10.14× | 10.96× |
| 100,000 | 0.438 | 228.10M | 0.412 | 242.73M | 3.741 | 8.53× | 9.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.208 | 2.02× |
| 1 | 5 | 0.309 | 0.937 | 3.03× |
| 1 | 10 | 0.398 | 2.162 | 5.43× |
| 10 | 1 | 0.049 | 0.192 | 3.94× |
| 10 | 5 | 0.182 | 0.926 | 5.09× |
| 10 | 10 | 0.411 | 2.136 | 5.20× |
| 100 | 1 | 0.045 | 0.197 | 4.41× |
| 100 | 5 | 0.214 | 0.962 | 4.50× |
| 100 | 10 | 0.397 | 2.243 | 5.65× |
| 1,000 | 1 | 0.055 | 0.225 | 4.11× |
| 1,000 | 5 | 0.208 | 1.117 | 5.38× |
| 1,000 | 10 | 0.473 | 2.596 | 5.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
