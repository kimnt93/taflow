# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.35M | 0.011 | 92.94M | 0.225 | 18.56× | 20.94× |
| 10,000 | 0.104 | 95.77M | 0.098 | 102.39M | 0.834 | 7.99× | 8.54× |
| 100,000 | 0.999 | 100.13M | 1.000 | 99.95M | 8.509 | 8.52× | 8.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.264 | 2.63× |
| 1 | 5 | 0.234 | 1.117 | 4.78× |
| 1 | 10 | 0.413 | 2.284 | 5.53× |
| 10 | 1 | 0.047 | 0.203 | 4.28× |
| 10 | 5 | 0.191 | 1.228 | 6.43× |
| 10 | 10 | 0.466 | 2.398 | 5.14× |
| 100 | 1 | 0.047 | 0.222 | 4.68× |
| 100 | 5 | 0.200 | 1.342 | 6.72× |
| 100 | 10 | 0.416 | 2.261 | 5.44× |
| 1,000 | 1 | 0.059 | 0.281 | 4.76× |
| 1,000 | 5 | 0.212 | 1.641 | 7.76× |
| 1,000 | 10 | 0.429 | 3.043 | 7.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
