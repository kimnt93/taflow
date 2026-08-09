# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.24M | 0.006 | 169.35M | 0.029 | 3.54× | 4.91× |
| 10,000 | 0.020 | 496.02M | 0.016 | 622.12M | 0.041 | 2.03× | 2.54× |
| 100,000 | 0.147 | 681.69M | 0.113 | 881.51M | 0.135 | 0.92× | 1.19× |
| 1,000,000 | 2.678 | 373.37M | 1.940 | 515.37M | 2.157 | 0.81× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.168 | 1.43× |
| 1 | 5 | 0.295 | 0.481 | 1.63× |
| 1 | 10 | 0.509 | 0.869 | 1.71× |
| 10 | 1 | 0.054 | 0.088 | 1.63× |
| 10 | 5 | 0.271 | 0.500 | 1.84× |
| 10 | 10 | 0.555 | 0.987 | 1.78× |
| 100 | 1 | 0.055 | 0.088 | 1.60× |
| 100 | 5 | 0.249 | 0.465 | 1.87× |
| 100 | 10 | 0.549 | 1.017 | 1.85× |
| 1,000 | 1 | 0.055 | 0.091 | 1.64× |
| 1,000 | 5 | 0.242 | 0.451 | 1.86× |
| 1,000 | 10 | 0.560 | 1.036 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
