# SignedPower benchmark (`numpy.sign/abs/power` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.78M | 0.005 | 219.68M | 0.027 | 4.25× | 6.04× |
| 10,000 | 0.023 | 436.17M | 0.020 | 499.84M | 0.046 | 1.99× | 2.28× |
| 100,000 | 0.196 | 511.07M | 0.174 | 574.13M | 0.205 | 1.05× | 1.18× |
| 1,000,000 | 2.144 | 466.44M | 1.800 | 555.47M | 2.984 | 1.39× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.104 | 0.85× |
| 1 | 5 | 0.350 | 0.431 | 1.23× |
| 1 | 10 | 0.474 | 0.859 | 1.81× |
| 10 | 1 | 0.045 | 0.087 | 1.94× |
| 10 | 5 | 0.224 | 0.415 | 1.85× |
| 10 | 10 | 0.457 | 0.840 | 1.84× |
| 100 | 1 | 0.050 | 0.089 | 1.80× |
| 100 | 5 | 0.219 | 0.417 | 1.90× |
| 100 | 10 | 0.456 | 0.875 | 1.92× |
| 1,000 | 1 | 0.047 | 0.090 | 1.89× |
| 1,000 | 5 | 0.220 | 0.452 | 2.05× |
| 1,000 | 10 | 0.502 | 1.007 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
