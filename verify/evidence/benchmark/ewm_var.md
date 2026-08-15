# ExponentiallyWeightedVariance benchmark (`ewm variance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.43M | 0.006 | 180.44M | 1.246 | 201.20× | 224.90× |
| 10,000 | 0.044 | 225.56M | 0.041 | 241.01M | 12.170 | 274.50× | 293.30× |
| 100,000 | 0.419 | 238.58M | 0.392 | 255.22M | 121.089 | 288.90× | 309.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.155 | 1.40× |
| 1 | 5 | 0.287 | 0.415 | 1.45× |
| 1 | 10 | 0.380 | 0.835 | 2.19× |
| 10 | 1 | 0.049 | 0.101 | 2.05× |
| 10 | 5 | 0.185 | 0.460 | 2.48× |
| 10 | 10 | 0.405 | 0.976 | 2.41× |
| 100 | 1 | 0.041 | 0.204 | 4.99× |
| 100 | 5 | 0.190 | 1.060 | 5.58× |
| 100 | 10 | 0.421 | 2.037 | 4.84× |
| 1,000 | 1 | 0.048 | 1.333 | 27.94× |
| 1,000 | 5 | 0.211 | 6.665 | 31.63× |
| 1,000 | 10 | 0.398 | 13.535 | 33.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
