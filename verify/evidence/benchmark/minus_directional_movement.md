# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.71M | 0.009 | 117.55M | 0.044 | 4.60× | 5.16× |
| 10,000 | 0.058 | 173.20M | 0.055 | 182.57M | 0.089 | 1.54× | 1.62× |
| 100,000 | 0.536 | 186.61M | 0.528 | 189.41M | 0.574 | 1.07× | 1.09× |
| 1,000,000 | 5.904 | 169.37M | 5.982 | 167.17M | 5.499 | 0.93× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.102 | 1.20× |
| 1 | 5 | 0.300 | 0.581 | 1.94× |
| 1 | 10 | 0.548 | 1.239 | 2.26× |
| 10 | 1 | 0.068 | 0.129 | 1.90× |
| 10 | 5 | 0.268 | 0.541 | 2.02× |
| 10 | 10 | 0.522 | 1.042 | 1.99× |
| 100 | 1 | 0.086 | 0.165 | 1.91× |
| 100 | 5 | 1.001 | 0.725 | 0.72× |
| 100 | 10 | 0.681 | 1.890 | 2.77× |
| 1,000 | 1 | 0.079 | 0.152 | 1.92× |
| 1,000 | 5 | 0.416 | 0.779 | 1.87× |
| 1,000 | 10 | 0.867 | 1.496 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
