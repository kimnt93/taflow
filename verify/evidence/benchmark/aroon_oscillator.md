# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.51M | 0.029 | 33.99M | 0.038 | 1.21× | 1.30× |
| 10,000 | 0.287 | 34.83M | 0.286 | 34.95M | 0.138 | 0.48× | 0.48× |
| 100,000 | 2.845 | 35.15M | 2.893 | 34.57M | 1.066 | 0.37× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.100 | 1.07× |
| 1 | 5 | 0.318 | 0.449 | 1.41× |
| 1 | 10 | 0.498 | 0.964 | 1.94× |
| 10 | 1 | 0.050 | 0.092 | 1.84× |
| 10 | 5 | 0.253 | 0.456 | 1.80× |
| 10 | 10 | 0.505 | 0.953 | 1.89× |
| 100 | 1 | 0.059 | 0.106 | 1.81× |
| 100 | 5 | 0.235 | 0.452 | 1.92× |
| 100 | 10 | 0.502 | 0.908 | 1.81× |
| 1,000 | 1 | 0.086 | 0.108 | 1.25× |
| 1,000 | 5 | 0.242 | 0.546 | 2.25× |
| 1,000 | 10 | 0.516 | 1.035 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
