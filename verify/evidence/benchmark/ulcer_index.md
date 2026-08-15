# UlcerIndex benchmark (`UlcerIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.48M | 0.018 | 55.78M | 0.197 | 11.11× | 10.97× |
| 10,000 | 0.191 | 52.30M | 0.184 | 54.26M | 0.590 | 3.09× | 3.20× |
| 100,000 | 1.888 | 52.96M | 1.962 | 50.96M | 4.700 | 2.49× | 2.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.245 | 2.99× |
| 1 | 5 | 0.342 | 1.326 | 3.88× |
| 1 | 10 | 0.385 | 2.310 | 6.00× |
| 10 | 1 | 0.049 | 0.221 | 4.47× |
| 10 | 5 | 0.189 | 1.322 | 6.98× |
| 10 | 10 | 0.407 | 2.359 | 5.79× |
| 100 | 1 | 0.048 | 0.227 | 4.75× |
| 100 | 5 | 0.219 | 1.392 | 6.35× |
| 100 | 10 | 0.439 | 2.399 | 5.47× |
| 1,000 | 1 | 0.068 | 0.264 | 3.91× |
| 1,000 | 5 | 0.218 | 1.725 | 7.93× |
| 1,000 | 10 | 0.447 | 2.854 | 6.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
