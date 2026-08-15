# QuartileBands benchmark (`QuartileBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.64M | 0.055 | 18.33M | 0.713 | 12.57× | 13.06× |
| 10,000 | 0.597 | 16.74M | 0.612 | 16.33M | 5.510 | 9.22× | 9.00× |
| 100,000 | 5.997 | 16.68M | 6.018 | 16.62M | 59.165 | 9.87× | 9.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.287 | 2.55× |
| 1 | 5 | 0.224 | 1.170 | 5.22× |
| 1 | 10 | 0.403 | 2.485 | 6.16× |
| 10 | 1 | 0.047 | 0.218 | 4.67× |
| 10 | 5 | 0.202 | 1.281 | 6.35× |
| 10 | 10 | 0.492 | 2.403 | 4.89× |
| 100 | 1 | 0.052 | 0.273 | 5.28× |
| 100 | 5 | 0.209 | 1.733 | 8.28× |
| 100 | 10 | 0.464 | 3.002 | 6.47× |
| 1,000 | 1 | 0.123 | 1.072 | 8.74× |
| 1,000 | 5 | 0.243 | 4.502 | 18.50× |
| 1,000 | 10 | 0.533 | 8.995 | 16.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
