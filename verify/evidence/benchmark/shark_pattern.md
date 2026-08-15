# SharkPattern benchmark (`Shark` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.50M | 0.007 | 134.95M | 0.229 | 22.60× | 30.96× |
| 10,000 | 0.102 | 98.46M | 0.094 | 106.15M | 1.409 | 13.87× | 14.96× |
| 100,000 | 0.958 | 104.34M | 0.941 | 106.27M | 13.075 | 13.64× | 13.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.218 | 2.02× |
| 1 | 5 | 0.262 | 0.822 | 3.14× |
| 1 | 10 | 0.414 | 1.739 | 4.20× |
| 10 | 1 | 0.045 | 0.171 | 3.81× |
| 10 | 5 | 0.207 | 1.094 | 5.28× |
| 10 | 10 | 0.398 | 1.745 | 4.38× |
| 100 | 1 | 0.049 | 0.183 | 3.74× |
| 100 | 5 | 0.205 | 1.141 | 5.55× |
| 100 | 10 | 0.431 | 1.864 | 4.32× |
| 1,000 | 1 | 0.067 | 0.299 | 4.45× |
| 1,000 | 5 | 0.210 | 1.822 | 8.69× |
| 1,000 | 10 | 0.456 | 3.048 | 6.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
