# CupAndHandle benchmark (`CupAndHandle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.78M | 0.012 | 81.53M | 0.235 | 16.43× | 19.20× |
| 10,000 | 0.098 | 102.32M | 0.093 | 107.63M | 1.446 | 14.79× | 15.56× |
| 100,000 | 0.930 | 107.57M | 0.904 | 110.64M | 13.415 | 14.43× | 14.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.208 | 2.38× |
| 1 | 5 | 0.438 | 0.831 | 1.90× |
| 1 | 10 | 0.506 | 1.748 | 3.45× |
| 10 | 1 | 0.058 | 0.171 | 2.94× |
| 10 | 5 | 0.248 | 1.189 | 4.79× |
| 10 | 10 | 0.576 | 1.813 | 3.15× |
| 100 | 1 | 0.059 | 0.178 | 3.00× |
| 100 | 5 | 0.260 | 1.182 | 4.55× |
| 100 | 10 | 0.534 | 1.966 | 3.68× |
| 1,000 | 1 | 0.069 | 0.298 | 4.33× |
| 1,000 | 5 | 0.258 | 1.754 | 6.79× |
| 1,000 | 10 | 0.609 | 3.039 | 4.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
