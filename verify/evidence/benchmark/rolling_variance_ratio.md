# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.171 | 5.84M | 0.172 | 5.83M | 0.371 | 2.17× | 2.16× |
| 10,000 | 1.908 | 5.24M | 1.856 | 5.39M | 2.349 | 1.23× | 1.27× |
| 100,000 | 17.898 | 5.59M | 18.019 | 5.55M | 20.613 | 1.15× | 1.14× |
| 1,000,000 | 176.838 | 5.65M | 176.593 | 5.66M | 226.073 | 1.28× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.302 | 2.23× |
| 1 | 5 | 0.270 | 1.451 | 5.38× |
| 1 | 10 | 0.500 | 2.875 | 5.75× |
| 10 | 1 | 0.052 | 0.247 | 4.75× |
| 10 | 5 | 0.259 | 1.463 | 5.64× |
| 10 | 10 | 0.526 | 2.777 | 5.28× |
| 100 | 1 | 0.066 | 0.258 | 3.92× |
| 100 | 5 | 0.265 | 1.504 | 5.68× |
| 100 | 10 | 0.541 | 2.979 | 5.50× |
| 1,000 | 1 | 0.247 | 0.494 | 2.01× |
| 1,000 | 5 | 0.445 | 2.710 | 6.09× |
| 1,000 | 10 | 0.673 | 5.157 | 7.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
