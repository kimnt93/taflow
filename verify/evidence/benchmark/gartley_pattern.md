# GartleyPattern benchmark (`Gartley` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.17M | 0.012 | 86.18M | 0.207 | 15.35× | 17.84× |
| 10,000 | 0.095 | 105.33M | 0.088 | 113.25M | 1.271 | 13.39× | 14.39× |
| 100,000 | 0.903 | 110.71M | 0.878 | 113.96M | 13.247 | 14.67× | 15.10× |
| 1,000,000 | 10.571 | 94.60M | 9.393 | 106.47M | 135.713 | 12.84× | 14.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.208 | 1.91× |
| 1 | 5 | 0.289 | 1.107 | 3.83× |
| 1 | 10 | 0.568 | 1.746 | 3.08× |
| 10 | 1 | 0.059 | 0.165 | 2.77× |
| 10 | 5 | 0.261 | 1.099 | 4.22× |
| 10 | 10 | 0.544 | 1.781 | 3.28× |
| 100 | 1 | 0.053 | 0.185 | 3.51× |
| 100 | 5 | 0.289 | 1.157 | 4.01× |
| 100 | 10 | 0.543 | 1.871 | 3.44× |
| 1,000 | 1 | 0.063 | 0.315 | 4.98× |
| 1,000 | 5 | 0.284 | 1.762 | 6.21× |
| 1,000 | 10 | 1.113 | 4.940 | 4.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
