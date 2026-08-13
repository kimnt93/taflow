# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.05M | 0.060 | 16.59M | 0.251 | 3.78× | 4.17× |
| 10,000 | 0.478 | 20.93M | 0.456 | 21.95M | 1.292 | 2.71× | 2.84× |
| 100,000 | 4.373 | 22.87M | 4.335 | 23.07M | 12.472 | 2.85× | 2.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.256 | 1.81× |
| 1 | 5 | 0.517 | 1.176 | 2.27× |
| 1 | 10 | 0.724 | 2.578 | 3.56× |
| 10 | 1 | 0.080 | 0.207 | 2.58× |
| 10 | 5 | 0.321 | 1.026 | 3.20× |
| 10 | 10 | 0.657 | 2.173 | 3.31× |
| 100 | 1 | 0.079 | 0.218 | 2.75× |
| 100 | 5 | 0.321 | 1.308 | 4.08× |
| 100 | 10 | 0.709 | 2.328 | 3.29× |
| 1,000 | 1 | 0.132 | 0.362 | 2.74× |
| 1,000 | 5 | 0.330 | 1.837 | 5.57× |
| 1,000 | 10 | 0.706 | 3.800 | 5.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
