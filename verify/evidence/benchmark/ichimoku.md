# Ichimoku benchmark (`causal ichimoku components` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 9.96M | 0.088 | 11.34M | 0.517 | 5.15× | 5.87× |
| 10,000 | 0.935 | 10.69M | 0.932 | 10.73M | 2.864 | 3.06× | 3.07× |
| 100,000 | 9.812 | 10.19M | 9.844 | 10.16M | 25.225 | 2.57× | 2.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.156 | 0.190 | 1.22× |
| 1 | 5 | 0.272 | 0.755 | 2.78× |
| 1 | 10 | 0.379 | 1.364 | 3.60× |
| 10 | 1 | 0.056 | 0.222 | 3.97× |
| 10 | 5 | 0.220 | 1.028 | 4.66× |
| 10 | 10 | 0.426 | 2.150 | 5.05× |
| 100 | 1 | 0.062 | 0.332 | 5.33× |
| 100 | 5 | 0.219 | 1.812 | 8.29× |
| 100 | 10 | 0.453 | 3.689 | 8.14× |
| 1,000 | 1 | 0.149 | 0.564 | 3.80× |
| 1,000 | 5 | 0.347 | 2.174 | 6.27× |
| 1,000 | 10 | 0.528 | 4.473 | 8.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
