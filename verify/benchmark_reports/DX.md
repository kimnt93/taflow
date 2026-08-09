# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.79M | 0.013 | 77.07M | 0.041 | 3.04× | 3.17× |
| 10,000 | 0.106 | 94.00M | 0.104 | 96.57M | 0.123 | 1.15× | 1.19× |
| 100,000 | 1.084 | 92.29M | 1.145 | 87.33M | 1.003 | 0.93× | 0.88× |
| 1,000,000 | 11.734 | 85.22M | 11.001 | 90.90M | 9.218 | 0.79× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.136 | 0.94× |
| 1 | 5 | 0.290 | 0.502 | 1.73× |
| 1 | 10 | 0.526 | 1.049 | 1.99× |
| 10 | 1 | 0.054 | 0.106 | 1.98× |
| 10 | 5 | 0.265 | 0.539 | 2.04× |
| 10 | 10 | 0.616 | 1.162 | 1.89× |
| 100 | 1 | 0.054 | 0.101 | 1.86× |
| 100 | 5 | 0.585 | 0.583 | 1.00× |
| 100 | 10 | 0.592 | 1.201 | 2.03× |
| 1,000 | 1 | 0.064 | 0.109 | 1.71× |
| 1,000 | 5 | 0.552 | 0.644 | 1.17× |
| 1,000 | 10 | 0.661 | 1.275 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
