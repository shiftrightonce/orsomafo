# Change Log

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
- Event handlers **must** now implment `Default` as well
  ```rust
  #[derive(Default)]
  struct MyEventHandler;
  ```