# BatPattern benchmark (`Bat` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.21M | 0.007 | 137.01M | 0.236 | 22.27× | 32.39× |
| 10,000 | 0.090 | 111.00M | 0.087 | 115.53M | 1.404 | 15.58× | 16.22× |
| 100,000 | 0.899 | 111.20M | 0.836 | 119.57M | 12.643 | 14.06× | 15.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.192 | 2.42× |
| 1 | 5 | 0.300 | 0.804 | 2.68× |
| 1 | 10 | 0.451 | 1.730 | 3.84× |
| 10 | 1 | 0.046 | 0.163 | 3.55× |
| 10 | 5 | 0.191 | 1.088 | 5.70× |
| 10 | 10 | 0.433 | 1.677 | 3.88× |
| 100 | 1 | 0.045 | 0.173 | 3.80× |
| 100 | 5 | 0.198 | 1.193 | 6.04× |
| 100 | 10 | 0.419 | 1.815 | 4.33× |
| 1,000 | 1 | 0.058 | 0.295 | 5.12× |
| 1,000 | 5 | 0.193 | 1.794 | 9.28× |
| 1,000 | 10 | 0.460 | 3.039 | 6.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
