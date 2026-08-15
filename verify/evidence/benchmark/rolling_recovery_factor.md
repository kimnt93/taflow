# RollingRecoveryFactor benchmark (`rolling recovery factor on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.67M | 0.050 | 20.01M | 0.215 | 4.45× | 4.31× |
| 10,000 | 0.480 | 20.84M | 0.471 | 21.21M | 1.337 | 2.79× | 2.84× |
| 100,000 | 4.874 | 20.52M | 4.769 | 20.97M | 17.048 | 3.50× | 3.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.113 | 1.57× |
| 1 | 5 | 0.283 | 0.446 | 1.58× |
| 1 | 10 | 0.380 | 1.035 | 2.73× |
| 10 | 1 | 0.048 | 0.091 | 1.90× |
| 10 | 5 | 0.186 | 0.404 | 2.18× |
| 10 | 10 | 0.394 | 0.870 | 2.21× |
| 100 | 1 | 0.053 | 0.184 | 3.50× |
| 100 | 5 | 0.200 | 0.995 | 4.98× |
| 100 | 10 | 0.436 | 1.879 | 4.31× |
| 1,000 | 1 | 0.090 | 0.291 | 3.25× |
| 1,000 | 5 | 0.255 | 1.095 | 4.29× |
| 1,000 | 10 | 0.466 | 2.289 | 4.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
