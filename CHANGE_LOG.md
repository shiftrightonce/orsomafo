# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1] - 2023-11-01

[890088f](890088fc8a23829cb2df823e429eade3a78d50f6)...[b011eec](b011eec92913675c975568772c9096ccc3e38618)

### Features

- Can now dispatch an serialised event ([46a4c47](46a4c47cc1bfd13f59e86cc12566a2778f71b1e0))
- Now expose some metadata ([342fee4](342fee4c064e4dae4fbf110c0a061da6e405e717))
- Add git-cliff config ([1d476da](1d476daeee97c895373aaa447baa219d8425d3bf))

### Refactor

- Change the `mpsc` channel data structure ([596c044](596c0442d35bc4d3dcc2a8cf080f1ec329a55883))


## 0.3.0
- Event must now implement `serde::Serialize`, `serde::Deserialize`
  We are working on a feature that will allow event to be broadcast to a "cluster"
  via a message broker like rabbitMQ or redis (pub/sub feature).

## 0.2.0
- Registering event handlers has changed.
  Initially, the to register an event handler you needed to do this:
  ```rust
  //..
  listen::<MyEvent>(MyEventHandler.to_handler())
  ```

  This has changed to
  ```rust
  //..
  listen::<MyEvent, MyEventHandler>()
  ```
- Event handlers **must** now implement `Default` as well
  ```rust
  #[derive(Default)]
  struct MyEventHandler;
  ```