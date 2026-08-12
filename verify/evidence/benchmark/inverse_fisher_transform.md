# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.13M | 0.006 | 169.49M | 0.163 | 24.58× | 27.56× |
| 10,000 | 0.040 | 247.95M | 0.038 | 262.85M | 0.473 | 11.74× | 12.44× |
| 100,000 | 0.359 | 278.63M | 0.337 | 296.41M | 3.532 | 9.84× | 10.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.244 | 3.39× |
| 1 | 5 | 0.316 | 1.280 | 4.05× |
| 1 | 10 | 0.497 | 2.243 | 4.52× |
| 10 | 1 | 0.047 | 0.209 | 4.42× |
| 10 | 5 | 0.237 | 1.264 | 5.34× |
| 10 | 10 | 0.528 | 2.264 | 4.29× |
| 100 | 1 | 0.055 | 0.216 | 3.95× |
| 100 | 5 | 0.245 | 1.275 | 5.21× |
| 100 | 10 | 0.452 | 2.346 | 5.19× |
| 1,000 | 1 | 0.060 | 0.259 | 4.30× |
| 1,000 | 5 | 0.235 | 1.422 | 6.06× |
| 1,000 | 10 | 0.506 | 2.672 | 5.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
