# template-runtime / crates/runtime

`template-runtime` is the workspace layer for concrete runtime holders and
resource-backed adapters.

It owns runtime context that can be shared by concrete implementations of core
ports. It does not define domain models, business rules, platform logging
backends, or demo-specific transports.

## Boundaries

- `base/*` defines foundational protocols and value types. `runtime` consumes
  those protocols, such as `george-base-log::SharedLogger`.
- `crates/model` defines public domain models. `runtime` should not place
  runtime resources or platform DTOs there.
- `crates/core` owns orchestration rules and port traits. Concrete port
  implementations belong in `runtime` when they need runtime resources.
- `crates/sdk` may re-export or assemble public APIs for callers. It should not
  become the concrete runtime implementation layer.
- `bindings` or application shells should bridge external platforms and
  languages. They should depend on runtime APIs instead of placing adapters in
  the model or core layers.

## Current Shape

The first version intentionally keeps only two public concepts:

- `RuntimeContext`
- `RuntimeBuilder`

`RuntimeContext` currently holds only a `SharedLogger`. `RuntimeBuilder` injects
that logger and falls back to `NoopLogger` when no logger is provided.

This crate is not a service locator. It should not grow a generic object
registry, global resource map, task manager, shutdown system, or transport
collection until a concrete runtime requirement proves that shape is necessary.

## Why Demo Features Are Not Here

Demo capabilities are not foundations for every future application built from
this template.

Putting demo transport or client code in the runtime skeleton would make the
template look like a product-specific runtime instead of a reusable runtime
holder. When a concrete feature needs an implementation, add a focused adapter
module that implements the corresponding `core` port and receives its resources
explicitly.

## Relationship With `george-platform-std`

`george-platform-std` owns the std/tracing logging backend: tracing subscriber
installation, file output, rollover, cleanup, and forwarding from
`george-base-log` records into tracing.

`template-runtime` does not install tracing and does not depend on
`george-platform-std`. It only accepts `SharedLogger`, so applications can choose
the platform logger they want and inject it through `RuntimeBuilder`.

## Adding Concrete Runtime Implementations Later

When a future feature needs runtime support:

1. Define the behavior boundary in `crates/core` as a port.
2. Implement the concrete adapter in `crates/runtime` or a dedicated runtime
   submodule.
3. Inject required resources through focused builders or constructors.
4. Keep demo transports, tokio handles, shutdown tokens, and platform resources
   out of `RuntimeContext` unless they are truly shared runtime-wide concerns.

The default should remain explicit resource ownership over broad dependency
containers.
