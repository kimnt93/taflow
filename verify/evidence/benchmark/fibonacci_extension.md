# FibonacciExtension benchmark (`FibExtension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.68M | 0.014 | 71.12M | 0.555 | 32.58× | 39.49× |
| 10,000 | 0.152 | 65.78M | 0.142 | 70.47M | 5.086 | 33.46× | 35.84× |
| 100,000 | 1.563 | 63.97M | 1.315 | 76.03M | 52.099 | 33.33× | 39.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.207 | 2.62× |
| 1 | 5 | 0.218 | 0.893 | 4.09× |
| 1 | 10 | 0.402 | 1.910 | 4.75× |
| 10 | 1 | 0.048 | 0.177 | 3.65× |
| 10 | 5 | 0.212 | 0.898 | 4.23× |
| 10 | 10 | 0.435 | 1.963 | 4.51× |
| 100 | 1 | 0.053 | 0.214 | 4.07× |
| 100 | 5 | 0.202 | 1.177 | 5.82× |
| 100 | 10 | 0.456 | 2.482 | 5.45× |
| 1,000 | 1 | 0.067 | 0.903 | 13.43× |
| 1,000 | 5 | 0.223 | 3.856 | 17.33× |
| 1,000 | 10 | 0.501 | 7.571 | 15.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
