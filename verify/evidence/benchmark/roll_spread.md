# RollSpread benchmark (`rolling Roll spread estimator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.02M | 0.046 | 21.64M | 0.258 | 5.68× | 5.58× |
| 10,000 | 0.426 | 23.50M | 0.429 | 23.31M | 1.239 | 2.91× | 2.89× |
| 100,000 | 4.203 | 23.79M | 4.397 | 22.74M | 13.511 | 3.21× | 3.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.130 | 1.27× |
| 1 | 5 | 0.334 | 0.565 | 1.69× |
| 1 | 10 | 0.490 | 1.392 | 2.84× |
| 10 | 1 | 0.049 | 0.117 | 2.38× |
| 10 | 5 | 0.230 | 0.537 | 2.33× |
| 10 | 10 | 0.476 | 1.094 | 2.30× |
| 100 | 1 | 0.053 | 0.234 | 4.42× |
| 100 | 5 | 0.260 | 1.183 | 4.55× |
| 100 | 10 | 0.504 | 2.498 | 4.95× |
| 1,000 | 1 | 0.097 | 0.345 | 3.56× |
| 1,000 | 5 | 0.263 | 1.469 | 5.58× |
| 1,000 | 10 | 0.572 | 3.202 | 5.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
