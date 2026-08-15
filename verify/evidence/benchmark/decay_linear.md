# DecayLinear benchmark (`linear decay weighted mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 205.90M | 0.004 | 274.23M | 0.080 | 16.45× | 21.91× |
| 10,000 | 0.033 | 306.78M | 0.029 | 339.97M | 0.288 | 8.84× | 9.79× |
| 100,000 | 0.302 | 331.42M | 0.284 | 352.69M | 2.083 | 6.90× | 7.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.143 | 1.22× |
| 1 | 5 | 0.248 | 0.559 | 2.25× |
| 1 | 10 | 0.406 | 1.233 | 3.04× |
| 10 | 1 | 0.047 | 0.122 | 2.62× |
| 10 | 5 | 0.189 | 0.550 | 2.92× |
| 10 | 10 | 0.378 | 1.099 | 2.91× |
| 100 | 1 | 0.048 | 0.158 | 3.28× |
| 100 | 5 | 0.186 | 0.710 | 3.82× |
| 100 | 10 | 0.390 | 1.479 | 3.79× |
| 1,000 | 1 | 0.043 | 0.168 | 3.89× |
| 1,000 | 5 | 0.187 | 0.774 | 4.14× |
| 1,000 | 10 | 0.404 | 1.726 | 4.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
