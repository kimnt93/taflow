# NegativeVolumeIndex benchmark (`NVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.68M | 0.007 | 142.48M | 0.218 | 25.25× | 31.09× |
| 10,000 | 0.063 | 159.58M | 0.060 | 167.69M | 0.793 | 12.65× | 13.29× |
| 100,000 | 0.587 | 170.46M | 0.562 | 177.91M | 7.053 | 12.02× | 12.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.254 | 2.21× |
| 1 | 5 | 0.262 | 1.110 | 4.23× |
| 1 | 10 | 0.531 | 2.418 | 4.55× |
| 10 | 1 | 0.059 | 0.231 | 3.95× |
| 10 | 5 | 0.224 | 1.324 | 5.90× |
| 10 | 10 | 0.509 | 2.386 | 4.69× |
| 100 | 1 | 0.053 | 0.234 | 4.44× |
| 100 | 5 | 0.239 | 1.405 | 5.88× |
| 100 | 10 | 0.540 | 2.419 | 4.48× |
| 1,000 | 1 | 0.071 | 0.302 | 4.27× |
| 1,000 | 5 | 0.256 | 1.746 | 6.83× |
| 1,000 | 10 | 0.525 | 3.001 | 5.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
