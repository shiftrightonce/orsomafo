# Orsomafo

Orsomafo is an event dispatcher for rust application

## Example (The long way)

```rust

use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
use tokio::time::{sleep, Duration};

 #[derive(Clone, Debug)] // Event must be cloneable
 struct MyEvent;

 impl orsomafo::Dispatchable for MyEvent {} // MyEvent is now dispatchable

  // create a handler
  struct MyEventHandler;
    
  #[orsomafo::async_trait]
   impl orsomafo::EventHandler for MyEventHandler {
        // called when event from "MyEvent" is dispatched
        async fn handle(&self, dispatched: &DispatchedEvent)  {
           let event: MyEvent = dispatched.the_event().unwrap();  // Get the instance of "MyEvent"
           println!("handled my event: {:#?}",event);
        }
    }

  #[tokio::main]
  async fn main() {
   _ =  EventDispatcherBuilder::new()
         .listen::<MyEvent>, MyEventHandler>()
         .build().await;

    let event = MyEvent;
    event.dispatch_event();

 }

```

## Example (The short way)
```rust

use async_trait::async_trait;
use orsomafo::{Dispatchable, DispatchedEvent, EventDispatcherBuilder, EventHandler};
use tokio::time::{sleep, Duration};

 #[derive(Clone, Debug)] // Event must be cloneable
 struct MyEvent;

 impl orsomafo::Dispatchable for MyEvent {} // MyEvent is now dispatchable

  // create a handler
  struct MyEventHandler;
    
  #[orsomafo::async_trait]
   impl orsomafo::EventHandler for MyEventHandler {
        // called when event from "MyEvent" is dispatched
        async fn handle(&self, dispatched: &DispatchedEvent)  {
           let event: MyEvent = dispatched.the_event().unwrap();  // Get the instance of "MyEvent"
           println!("handled my event: {:#?}",event);
        }
    }

  #[tokio::main]
  async fn main() {

   MyEvent.subscribe::<MyEventHandler>().await;

    let event = MyEvent;
    event.dispatch_event();

 }

```

## Examples

The [examples](https://github.com/shiftrightonce/orsomafo/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.

## Feedback

If you find this crate useful, please star the repository. Submit your issues and recommendations as well.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
