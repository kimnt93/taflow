# Continuous-only interface review

## Scope

TAFlow's primary contract is a persistent, causal indicator object:
`append(...)`, `extend(...)`, `value`/`compute()`, and `reset()`. The root
`taflow` namespace exports the canonical CamelCase indicator classes, while
`taflow.executions` owns graph and container helpers.

## Current evidence

| Layer | Current surface | Review result |
|---|---:|---|
| Rust stream kernels | 493 public stream exports | persistent states and batch helpers coexist; states are the required path |
| PyO3 state/indicator classes | 283 discovered classes | used by canonical Python adapters |
| PyO3 one-shot functions | 0 registered functions | former `func_api` module is not compiled into the extension |
| Root `taflow` exports | 251 names | no callable snake_case functions; no uppercase TA-Lib functions |
| `taflow.executions` | 15 canonical helpers | `TAPipeline`, `TAExpr`, adapters, conversions, and helper wrappers |
| `taflow.talib` | `MaType`/`MA_Type` only | no one-shot functions; persistent aliases are isolated in `taflow.talib.state` |

## Target contract

1. `taflow`: CamelCase persistent indicator classes only (plus the `talib`
   module handle, `MaType`, version metadata, and `executions` namespace).
2. `taflow.executions`: explicitly named execution/adaptation helpers.
3. `taflow.talib`: no one-shot uppercase functions, registry, abstract API, or
   stream recomputation wrappers. If retained, it contains only optional
   state aliases for migration and is not used by canonical code.
4. Rust/PyO3 one-shot `func_api` bindings are not part of the public extension
   module; persistent state classes remain registered.

## Migration checks

- [x] Canonical Python adapters use native state/indicator classes.
- [x] Root namespace no longer exports one-shot helpers.
- [x] Execution helpers moved to `taflow.executions`.
- [x] Remove the remaining `taflow.talib` batch/stream/abstract surface.
- [x] Remove one-shot `func_api` registration from the PyO3 module.
- [x] Update verification and benchmark runners to use canonical states and
      the independent external TA-Lib package only when an oracle is desired.
