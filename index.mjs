import init, { Foo, set_hook, do_panic } from "./pkg/wasm_test.js";

(async function main() {
  await init();
  set_hook();

  const foo = new Foo();

  // This causes "recursive use of an object detected" error
  foo.start("Amagi");

  // If I await the method call, it won't cause the "recursive" error
  // await foo.start("Amagi");

  // This doesn't cause "recursive" error
  // do_panic();

  let i = 0;
  (function loop() {
    console.log(">> loop", i++);
    foo.hello();
    requestAnimationFrame(loop);
  })();
})();
