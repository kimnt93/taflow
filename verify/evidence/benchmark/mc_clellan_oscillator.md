# McClellanOscillator benchmark (`McClellanOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.53M | 0.043 | 23.39M | 7.945 | 163.08× | 185.85× |
| 10,000 | 0.352 | 28.40M | 0.348 | 28.77M | 78.994 | 224.36× | 227.30× |
| 100,000 | 3.349 | 29.86M | 3.272 | 30.57M | 779.334 | 232.72× | 238.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.200 | 0.261 | 1.30× |
| 1 | 5 | 0.409 | 1.090 | 2.66× |
| 1 | 10 | 0.608 | 2.102 | 3.46× |
| 10 | 1 | 0.072 | 0.296 | 4.14× |
| 10 | 5 | 0.297 | 1.708 | 5.74× |
| 10 | 10 | 0.628 | 2.859 | 4.56× |
| 100 | 1 | 0.071 | 1.044 | 14.75× |
| 100 | 5 | 0.283 | 5.565 | 19.68× |
| 100 | 10 | 0.617 | 10.739 | 17.40× |
| 1,000 | 1 | 0.110 | 8.415 | 76.29× |
| 1,000 | 5 | 0.423 | 50.230 | 118.81× |
| 1,000 | 10 | 0.927 | 116.749 | 125.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
