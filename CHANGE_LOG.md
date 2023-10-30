# Change Log

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