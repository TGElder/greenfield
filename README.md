## Testing

Run `./test.sh`

### Graphics engine tests

New tests must be added to `./engine/test.sh`. The script runs tests individually because
   1. winit only allows a single EventLoop to be created per run
   2. each test needs an EventLoop and Graphics struct
   3. neither EventLoop nor Graphics can be shared between tests (lazy_static does not work because the structs are not Sync)

These tests run in headed mode - glium removed its headless mode in 0.30. This means you will see windows open as the tests run. The tests do not run on Github for this reason.
