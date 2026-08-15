# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.20M | 0.009 | 115.93M | 0.038 | 3.84× | 4.44× |
| 10,000 | 0.115 | 87.24M | 0.111 | 89.98M | 0.138 | 1.20× | 1.24× |
| 100,000 | 1.151 | 86.89M | 1.123 | 89.06M | 1.087 | 0.94× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.117 | 1.76× |
| 1 | 5 | 0.224 | 0.456 | 2.03× |
| 1 | 10 | 0.417 | 0.982 | 2.36× |
| 10 | 1 | 0.049 | 0.094 | 1.92× |
| 10 | 5 | 0.180 | 0.439 | 2.44× |
| 10 | 10 | 0.381 | 0.935 | 2.46× |
| 100 | 1 | 0.048 | 0.095 | 2.00× |
| 100 | 5 | 0.209 | 0.434 | 2.08× |
| 100 | 10 | 0.429 | 0.945 | 2.20× |
| 1,000 | 1 | 0.055 | 0.096 | 1.74× |
| 1,000 | 5 | 0.193 | 0.502 | 2.60× |
| 1,000 | 10 | 0.426 | 1.058 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
