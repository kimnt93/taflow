# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.74M | 0.006 | 177.27M | 0.145 | 22.84× | 25.67× |
| 10,000 | 0.046 | 216.49M | 0.043 | 230.30M | 0.466 | 10.09× | 10.74× |
| 100,000 | 0.439 | 227.91M | 0.433 | 230.95M | 3.406 | 7.76× | 7.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.218 | 1.38× |
| 1 | 5 | 0.249 | 0.925 | 3.71× |
| 1 | 10 | 0.421 | 2.136 | 5.08× |
| 10 | 1 | 0.049 | 0.184 | 3.77× |
| 10 | 5 | 0.188 | 0.937 | 4.99× |
| 10 | 10 | 0.461 | 2.121 | 4.60× |
| 100 | 1 | 0.043 | 0.189 | 4.40× |
| 100 | 5 | 0.208 | 0.967 | 4.64× |
| 100 | 10 | 0.436 | 2.130 | 4.89× |
| 1,000 | 1 | 0.047 | 0.218 | 4.66× |
| 1,000 | 5 | 0.205 | 1.193 | 5.82× |
| 1,000 | 10 | 0.443 | 2.437 | 5.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
