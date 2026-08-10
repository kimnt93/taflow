# TripleTopBottom benchmark (`TripleTopBottom` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.88M | 0.011 | 89.55M | 0.257 | 19.21× | 22.98× |
| 10,000 | 0.096 | 104.38M | 0.089 | 112.64M | 1.414 | 14.76× | 15.93× |
| 100,000 | 0.899 | 111.28M | 1.003 | 99.71M | 12.880 | 14.33× | 12.84× |
| 1,000,000 | 9.152 | 109.26M | 10.050 | 99.50M | 127.863 | 13.97× | 12.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.201 | 2.15× |
| 1 | 5 | 0.357 | 1.083 | 3.03× |
| 1 | 10 | 0.524 | 1.671 | 3.19× |
| 10 | 1 | 0.060 | 0.176 | 2.91× |
| 10 | 5 | 0.255 | 1.097 | 4.30× |
| 10 | 10 | 0.793 | 2.424 | 3.06× |
| 100 | 1 | 0.059 | 0.193 | 3.25× |
| 100 | 5 | 0.271 | 1.152 | 4.25× |
| 100 | 10 | 0.539 | 1.801 | 3.34× |
| 1,000 | 1 | 0.066 | 0.306 | 4.66× |
| 1,000 | 5 | 0.262 | 1.719 | 6.56× |
| 1,000 | 10 | 0.561 | 2.994 | 5.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
