# ZeroLagExponentialMovingAverage benchmark (`ZLEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.99M | 0.007 | 139.93M | 0.159 | 19.21× | 22.22× |
| 10,000 | 0.048 | 210.20M | 0.062 | 162.16M | 0.929 | 19.52× | 15.06× |
| 100,000 | 0.729 | 137.09M | 0.823 | 121.49M | 3.961 | 5.43× | 4.81× |
| 1,000,000 | 4.877 | 205.03M | 4.255 | 235.04M | 39.457 | 8.09× | 9.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.171 | 0.552 | 3.23× |
| 1 | 5 | 0.284 | 1.238 | 4.35× |
| 1 | 10 | 0.570 | 2.463 | 4.32× |
| 10 | 1 | 0.070 | 0.238 | 3.41× |
| 10 | 5 | 0.430 | 2.581 | 5.99× |
| 10 | 10 | 1.757 | 2.353 | 1.34× |
| 100 | 1 | 0.056 | 0.213 | 3.82× |
| 100 | 5 | 0.285 | 1.170 | 4.10× |
| 100 | 10 | 0.627 | 2.507 | 4.00× |
| 1,000 | 1 | 0.058 | 0.237 | 4.09× |
| 1,000 | 5 | 0.249 | 1.192 | 4.78× |
| 1,000 | 10 | 0.544 | 3.108 | 5.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
