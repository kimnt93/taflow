# ValueWhen benchmark (`last value when condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.77M | 0.030 | 33.46M | 0.144 | 3.99× | 4.80× |
| 10,000 | 0.218 | 45.82M | 0.209 | 47.78M | 1.324 | 6.07× | 6.33× |
| 100,000 | 2.086 | 47.94M | 1.993 | 50.19M | 13.094 | 6.28× | 6.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.108 | 0.94× |
| 1 | 5 | 0.433 | 0.314 | 0.73× |
| 1 | 10 | 0.615 | 0.656 | 1.07× |
| 10 | 1 | 0.063 | 0.067 | 1.07× |
| 10 | 5 | 0.289 | 0.319 | 1.10× |
| 10 | 10 | 0.585 | 0.659 | 1.13× |
| 100 | 1 | 0.066 | 0.079 | 1.19× |
| 100 | 5 | 0.295 | 0.384 | 1.30× |
| 100 | 10 | 0.620 | 0.797 | 1.29× |
| 1,000 | 1 | 0.090 | 0.211 | 2.35× |
| 1,000 | 5 | 0.286 | 0.997 | 3.49× |
| 1,000 | 10 | 0.611 | 2.047 | 3.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
