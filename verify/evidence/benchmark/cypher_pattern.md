# CypherPattern benchmark (`Cypher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.96M | 0.011 | 92.67M | 0.233 | 16.07× | 21.60× |
| 10,000 | 0.088 | 113.68M | 0.081 | 122.92M | 1.383 | 15.73× | 17.00× |
| 100,000 | 0.814 | 122.91M | 0.778 | 128.62M | 12.736 | 15.65× | 16.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.213 | 2.98× |
| 1 | 5 | 0.348 | 0.861 | 2.47× |
| 1 | 10 | 0.518 | 1.698 | 3.28× |
| 10 | 1 | 0.063 | 0.193 | 3.06× |
| 10 | 5 | 0.248 | 1.166 | 4.70× |
| 10 | 10 | 0.533 | 1.741 | 3.27× |
| 100 | 1 | 0.061 | 0.179 | 2.94× |
| 100 | 5 | 0.275 | 1.190 | 4.33× |
| 100 | 10 | 0.575 | 1.900 | 3.31× |
| 1,000 | 1 | 0.071 | 0.311 | 4.40× |
| 1,000 | 5 | 0.291 | 1.794 | 6.15× |
| 1,000 | 10 | 0.568 | 3.049 | 5.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
